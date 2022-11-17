use std::{collections::BTreeMap, ffi::OsString, path::PathBuf};

use crate::{
    file_tree::FileTree,
    templates::{entry_type::scaffold_entry_type_templates, ScaffoldedTemplate},
    utils::{input_snake_case, input_snake_case_with_initial_text},
};

use build_fs_tree::dir;
use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect, Select};
use holochain_types::prelude::{AppManifest, DnaManifest, ZomeManifest};
use serde::{Deserialize, Serialize};

use crate::error::{ScaffoldError, ScaffoldResult};

use self::{
    coordinator::{add_crud_functions_to_coordinator, updates_link_name},
    crud::Crud,
    definitions::{
        Cardinality, DependsOn, EntryDefinition, EntryTypeReference, FieldDefinition, FieldType,
        Referenceable,
    },
    fields::choose_fields,
    integrity::{add_entry_type_to_integrity_zome, get_all_entry_types},
    utils::{choose_fixed, choose_reference_entry_hash, choose_referenceable},
};

use super::{
    app::utils::read_app_manifest,
    link_type::{integrity::add_link_type_to_integrity_zome, link_type_name},
    tryorama::add_tryorama_tests_for_entry_def,
    zome::{
        coordinator::find_extern_function_or_choose, utils::get_coordinator_zomes_for_integrity,
        ZomeFileTree,
    },
};

pub mod coordinator;
pub mod crud;
pub mod definitions;
pub mod fields;
pub mod integrity;
pub mod utils;

fn choose_cardinality(referenceable: &Referenceable) -> ScaffoldResult<Cardinality> {
    let hash_type = referenceable.hash_type().to_string();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select the type of dependency to that entry type:")
        .default(0)
        .item(format!("Single ({})", hash_type))
        .item(format!("Optional (Option<{}>)", hash_type))
        .item(format!("Vector (Vec<{}>)", hash_type))
        .interact()?;

    match selection {
        0 => Ok(Cardinality::Single),
        1 => Ok(Cardinality::Option),
        _ => Ok(Cardinality::Vector),
    }
}

fn choose_depends_on(entry_types: &Vec<EntryTypeReference>) -> ScaffoldResult<Vec<DependsOn>> {
    let mut finished = false;

    let mut depends_on: Vec<DependsOn> = Vec::new();

    while !finished {
        let referenceable = choose_referenceable(
            &entry_types,
            &String::from("Select an existing entry type that the new entry type depends on:"),
        )?;
        let cardinality = choose_cardinality(&referenceable)?;

        let field_name = referenceable.field_name(&cardinality);

        depends_on.push(DependsOn {
            referenceable,
            cardinality,
            field_name,
        });

        println!("");

        finished = !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Does the new entry type depend on another existing entry type?")
            .interact()?;
    }

    println!("");

    Ok(depends_on)
}

fn get_or_choose_depends_on(
    zome_file_tree: &ZomeFileTree,
    depends_on: &Option<Vec<Referenceable>>,
) -> ScaffoldResult<Vec<DependsOn>> {
    let entry_types = get_all_entry_types(zome_file_tree)?.unwrap_or_else(|| vec![]);

    let entry_types_names: Vec<String> = entry_types
        .clone()
        .into_iter()
        .map(|et| et.entry_type.clone())
        .collect();

    match depends_on {
        Some(et) => match et
            .iter()
            .filter_map(|t| match t {
                Referenceable::Agent { .. } => None,
                Referenceable::EntryType(et) => Some(et),
            })
            .find(|t| !entry_types_names.contains(&t.entry_type))
        {
            Some(t) => Err(ScaffoldError::EntryTypeNotFound(
                t.entry_type.clone(),
                zome_file_tree.dna_file_tree.dna_manifest.name(),
                zome_file_tree.zome_manifest.name.0.to_string(),
            )),
            None => Ok(et
                .clone()
                .into_iter()
                .map(|referenceable| {
                    let cardinality = Cardinality::Single;
                    DependsOn {
                        referenceable: referenceable.clone(),
                        field_name: referenceable.field_name(&cardinality),
                        cardinality,
                    }
                })
                .collect()),
        },
        None => {
            println!(
                r#"
In most holochain apps, there are dependency relationships between entry types. 

An entry type "B" depends on an entry type "A" when:

    1. There is a field in entry type "B" that contains hashes referencing entries from the "A" entry type.
    2. There is a link from those entries of type "A" to the entry of type "B" that references them.
                
For example, in a forum app, the "comment" entry type depends on the "post" entry type.
"#
            );
            let depends = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Does the new entry type depend on an existing one?")
                .interact()?;
            match depends {
                true => choose_depends_on(&entry_types),
                false => Ok(Vec::new()),
            }
        }
    }
}

pub fn choose_depends_on_itself(name: &String) -> ScaffoldResult<DependsOnItself> {
    let depends = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Does a new entry of this type depend on previously existing entries of its same type?",
        )
        .interact()?;
    if !depends {
        return Ok(DependsOnItself::None);
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Does an entry of this type depend on an Option or a Vector of entries of this type?",
        )
        .default(0)
        .item(format!("Option ({}_hash: Option<ActionHash>)", name))
        .item(format!(
            "Vector ({}_hashes: Vec<ActionHash>)",
            pluralizer::pluralize(name, 2, false).to_case(Case::Snake)
        ))
        .interact()?;

    match selection {
        0 => Ok(DependsOnItself::Some(SelfDependencyCardinality::Option)),
        _ => Ok(DependsOnItself::Some(SelfDependencyCardinality::Vector)),
    }
}

pub type DependsOnItself = Option<SelfDependencyCardinality>;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum SelfDependencyCardinality {
    Vector,
    Option,
}

impl Into<Cardinality> for SelfDependencyCardinality {
    fn into(self) -> Cardinality {
        match self {
            SelfDependencyCardinality::Vector => Cardinality::Vector,
            SelfDependencyCardinality::Option => Cardinality::Option,
        }
    }
}

pub fn parse_depends_on_itself(depends_on_itself: &str) -> Result<DependsOnItself, String> {
    match depends_on_itself {
        "false" => Ok(DependsOnItself::None),
        "vector" => Ok(DependsOnItself::Some(SelfDependencyCardinality::Vector)),
        "option" => Ok(DependsOnItself::Some(SelfDependencyCardinality::Option)),
        _ => Err(format!(
            "Invalid depends_on_itself value {}. Valid values: \"false\", \"vector\", \"option\" ",
            depends_on_itself
        )),
    }
}

pub fn depends_on_itself_field_name(
    name: &String,
    cardinality: &SelfDependencyCardinality,
) -> String {
    match cardinality.clone().into() {
        Cardinality::Vector => format!(
            "previous_{}_hashes",
            pluralizer::pluralize(name, 2, false).to_case(Case::Snake)
        ),
        _ => format!("{}_hash", name.to_case(Case::Snake)),
    }
}

pub fn depends_on_field_definition(depends_on: &DependsOn) -> FieldDefinition {
    FieldDefinition {
        field_name: depends_on.field_name.clone(),
        field_type: depends_on.referenceable.hash_type(),
        widget: None,
        cardinality: depends_on.cardinality.clone(),
    }
}

pub fn scaffold_entry_type(
    zome_file_tree: ZomeFileTree,
    template_file_tree: &FileTree,
    name: &String,
    maybe_crud: &Option<Crud>,
    maybe_fixed: &Option<bool>,
    maybe_link_from_original_to_each_update: &Option<bool>,
    maybe_depends_on: &Option<Vec<Referenceable>>,
    maybe_depends_on_itself: &Option<DependsOnItself>,
    maybe_fields: &Option<Vec<(String, FieldType)>>,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let depends_on = get_or_choose_depends_on(&zome_file_tree, maybe_depends_on)?;
    let depends_on_itself: DependsOnItself = match maybe_depends_on_itself {
        Some(d) => d.clone(),
        None => choose_depends_on_itself(name)?,
    };

    let mut depends_fields: Vec<FieldDefinition> = Vec::new();
    for d in depends_on.clone() {
        depends_fields.push(depends_on_field_definition(&d));
    }

    if let DependsOnItself::Some(cardinality) = depends_on_itself.clone() {
        depends_fields.push(FieldDefinition {
            widget: None,
            field_name: depends_on_itself_field_name(&name, &cardinality),
            cardinality: cardinality.into(),
            field_type: FieldType::ActionHash,
        });
    }

    let fields = match maybe_fields {
        Some(f) => {
            for (field_name, field_type) in f {
                depends_fields.push(FieldDefinition {
                    field_name: field_name.clone(),
                    widget: None,
                    cardinality: Cardinality::Single,
                    field_type: field_type.clone(),
                });
            }

            depends_fields
        }
        None => {
            let v: Vec<OsString> = PathBuf::from("field-types")
                .iter()
                .map(|s| s.to_os_string())
                .collect();
            let empty_dir = dir! {};
            choose_fields(
                template_file_tree.path(&mut v.iter()).unwrap_or(&empty_dir),
                depends_fields,
            )?
        }
    };

    let fixed = match maybe_fixed {
        Some(r) => r.clone(),
        None => choose_fixed()?,
    };

    let (crud, link_from_original_to_each_update) = match fixed {
        true => (
            Crud {
                update: false,
                delete: false,
            },
            false,
        ),
        false => {
            let crud = match maybe_crud {
                Some(c) => c.clone(),
                None => choose_crud(),
            };

            let link_from_original_to_each_update = match crud.update {
                true => match maybe_link_from_original_to_each_update {
                    Some(l) => l.clone(),
                    None => {
                        let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Should a link from the original entry be created when this entry is updated?")
                .default(0)
                .item("Yes (more storage cost but better read performance, recommended)")
                .item("No (less storage cost but worse read performance)")
                .interact()?;

                        selection == 0
                    }
                },
                false => false,
            };
            (crud, link_from_original_to_each_update)
        }
    };

    let entry_def = EntryDefinition {
        name: name.clone(),
        fields,
        depends_on: depends_on.clone(),
        depends_on_itself: depends_on_itself.clone(),
        fixed,
    };

    let integrity_zome_name = zome_file_tree.zome_manifest.name.0.to_string();

    let mut zome_file_tree = add_entry_type_to_integrity_zome(zome_file_tree, &entry_def)?;

    for d in depends_on.clone() {
        zome_file_tree = add_link_type_to_integrity_zome(
            zome_file_tree,
            &link_type_name(&d.referenceable, &entry_def.referenceable()),
        )?;
    }
    if depends_on_itself.is_some() {
        zome_file_tree = add_link_type_to_integrity_zome(
            zome_file_tree,
            &link_type_name(&entry_def.referenceable(), &entry_def.referenceable()),
        )?;
    }

    let coordinator_zomes_for_integrity = get_coordinator_zomes_for_integrity(
        &zome_file_tree.dna_file_tree.dna_manifest,
        &zome_file_tree.zome_manifest.name.0.to_string(),
    );

    let coordinator_zome = match coordinator_zomes_for_integrity.len() {
        0 => Err(ScaffoldError::NoCoordinatorZomesFoundForIntegrityZome(
            zome_file_tree.dna_file_tree.dna_manifest.name(),
            zome_file_tree.zome_manifest.name.0.to_string(),
        )),
        1 => Ok(coordinator_zomes_for_integrity[0].clone()),
        _ => {
            let names: Vec<String> = coordinator_zomes_for_integrity
                .iter()
                .map(|z| z.name.0.to_string())
                .collect();
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Which coordinator zome should the CRUD functions be scaffolded in?")
                .default(0)
                .items(&names[..])
                .interact()?;

            Ok(coordinator_zomes_for_integrity[selection].clone())
        }
    }?;

    if link_from_original_to_each_update {
        zome_file_tree =
            add_link_type_to_integrity_zome(zome_file_tree, &updates_link_name(&entry_def.name))?;
    }

    let zome_file_tree =
        ZomeFileTree::from_zome_manifest(zome_file_tree.dna_file_tree, coordinator_zome.clone())?;

    let zome_file_tree = add_crud_functions_to_coordinator(
        zome_file_tree,
        &integrity_zome_name,
        &entry_def,
        &crud,
        link_from_original_to_each_update,
    )?;

    let mut create_fns_for_depends_on: BTreeMap<String, (ZomeManifest, String)> = BTreeMap::new();

    for d in depends_on.clone() {
        if let Referenceable::EntryType(entry_type_reference) = d.referenceable {
            let (zome, fn_name) = find_extern_function_or_choose(
                &zome_file_tree.dna_file_tree,
                &coordinator_zomes_for_integrity,
                &format!(
                    "create_{}",
                    entry_type_reference.entry_type.to_case(Case::Snake)
                ),
                &format!(
                    "In which function is a {} created?",
                    entry_type_reference.entry_type.to_case(Case::Pascal)
                ),
            )?;

            create_fns_for_depends_on.insert(
                entry_type_reference.entry_type.clone(),
                (zome, fn_name.sig.ident.to_string()),
            );
        }
    }

    let dna_manifest = zome_file_tree.dna_file_tree.dna_manifest.clone();

    let app_file_tree = add_tryorama_tests_for_entry_def(
        zome_file_tree,
        &entry_def,
        &crud,
        link_from_original_to_each_update,
        &create_fns_for_depends_on,
    )?;

    scaffold_entry_type_templates(
        app_file_tree,
        template_file_tree,
        &dna_manifest.name(),
        &coordinator_zome,
        &entry_def,
        &crud,
        link_from_original_to_each_update,
    )
}

fn choose_crud() -> Crud {
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which CRUD functions should be scaffolded (SPACE to select/unselect, ENTER to continue)?")
        .item_checked("Update", true)
        .item_checked("Delete", true)
        .interact()
        .unwrap();

    let mut crud = Crud {
        delete: false,

        update: false,
    };

    for selection in selections {
        if selection == 0 {
            crud.update = true;
        }
        if selection == 1 {
            crud.delete = true;
        }
    }

    crud
}
