use std::{collections::BTreeMap, path::PathBuf};

use crate::{
    definitions::{EntryDefinition, FieldDefinition, FieldType},
    file_tree::FileTree,
};
use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Confirm, MultiSelect, Select};
use holochain_types::prelude::{AppManifest, DnaManifest, ZomeManifest};
use serde::{Deserialize, Serialize};

use crate::error::{ScaffoldError, ScaffoldResult};

use self::{
    coordinator::add_crud_functions_to_coordinator,
    crud::Crud,
    fields::choose_fields,
    integrity::{add_entry_type_to_integrity_zome, get_all_entry_types},
    utils::choose_multiple_entry_types,
};

use super::{
    app::utils::read_app_manifest,
    link_type::{integrity::add_link_type_to_integrity_zome, link_type_name},
    tryorama::add_tryorama_tests_for_entry_def,
    web_app::uis::scaffold_entry_type_templates,
    zome::{
        coordinator::find_extern_function_or_choose, utils::get_coordinator_zomes_for_integrity,
        ZomeFileTree,
    },
};

pub mod coordinator;
pub mod crud;
pub mod fields;
pub mod integrity;
pub mod utils;

fn get_or_choose_depends_on(
    zome_file_tree: &ZomeFileTree,
    depends_on: &Option<Vec<String>>,
) -> ScaffoldResult<Vec<String>> {
    let entry_types = get_all_entry_types(zome_file_tree)?.unwrap_or_else(|| vec![]);

    if entry_types.len() == 0 {
        return Ok(vec![]);
    }

    match depends_on {
        Some(et) => match et.iter().find(|t| !entry_types.contains(t)) {
            Some(t) => Err(ScaffoldError::EntryTypeNotFound(
                t.clone(),
                zome_file_tree.dna_file_tree.dna_manifest.name(),
                zome_file_tree.zome_manifest.name.0.to_string(),
            )),
            None => Ok(et.clone()),
        },
        None => {
            let depends = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Does the new entry type depend on an existing one? (Eg. in a forum app, a comment depends on a post)")
                .interact()?;
            match depends {
                true => choose_multiple_entry_types(
                    &entry_types,
                    &String::from("Which existing entry types does the new entry type depend on?"),
                    false,
                ),
                false => Ok(vec![]),
            }
        }
    }
}

pub fn choose_depends_on_itself() -> ScaffoldResult<DependsOnItself> {
    let depends = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Does a new entry of this type depend on a existing entries of its same type? (Eg. in git, a commit depends on a list of previous_commits)")
                .interact()?;
    if !depends {
        return Ok(DependsOnItself::No);
    }

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Does an entry depend on an Option or a Vector of entries of its same entry type?",
        )
        .default(0)
        .item("Option")
        .item("Vector")
        .interact()?;

    match selection {
        0 => Ok(DependsOnItself::Yes(SelfDependencyType::Option)),
        1 => Ok(DependsOnItself::Yes(SelfDependencyType::Vector)),
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum DependsOnItself {
    No,
    Yes(SelfDependencyType),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SelfDependencyType {
    Vector,
    Option,
}

pub fn parse_depends_on_itself(depends_on_itself: &str) -> Result<DependsOnItself, String> {
    match depends_on_itself {
        "false" => Ok(DependsOnItself::No),
        "vector" => Ok(DependsOnItself::Yes(SelfDependencyType::Vector)),
        "option" => Ok(DependsOnItself::Yes(SelfDependencyType::Option)),
        _ => Err(format!(
            "Invalid depends_on_itself value {}. Valid values: \"false\", \"vector\", \"option\" ",
            depends_on_itself
        )),
    }
}

pub fn scaffold_entry_type(
    zome_file_tree: ZomeFileTree,
    template_file_tree: &FileTree,
    singular_name: &String,
    plural_name: &String,
    maybe_crud: &Option<Crud>,
    maybe_depends_on: &Option<Vec<String>>,
    maybe_depends_on_itself: &Option<DependsOnItself>,
    maybe_fields: &Option<Vec<(String, FieldType)>>,
) -> ScaffoldResult<FileTree> {
    let depends_on: Vec<String> = get_or_choose_depends_on(&zome_file_tree, maybe_depends_on)?;
    let depends_on_itself: DependsOnItself = match maybe_depends_on_itself {
        Some(d) => d.clone(),
        None => choose_depends_on_itself()?,
    };

    let mut depends_fields: Vec<(String, FieldDefinition)> = Vec::new();
    for d in depends_on.clone() {
        let field_name = format!("{}_hash", d.to_case(Case::Snake));
        depends_fields.push((
            field_name,
            FieldDefinition {
                widget: None,
                vector: false,
                field_type: FieldType::ActionHash,
            },
        ));
    }

    if let DependsOnItself::Yes(dependency_type) = depends_on_itself {
        if let SelfDependencyType::Vector = dependency_type {
        } else {
            let field_name = format!("previous_{}_hash", entry_def_name.to_case(Case::Snake));
            depends_fields.push((
                field_name,
                FieldDefinition {
                    widget: None,
                    vector: false,
                    field_type: FieldType::ActionHash,
                },
            ));
        }
    }

    let fields = match maybe_fields {
        Some(f) => {
            for (field_name, field_type) in f {
                depends_fields.push((
                    field_name.clone(),
                    FieldDefinition {
                        widget: None,
                        vector: false,
                        field_type: field_type.clone(),
                    },
                ));
            }

            depends_fields
        }
        None => choose_fields(
            path(templates_file_tree, &PathBuf::from("field-types")),
            depends_fields,
        )?,
    };

    let entry_def = EntryDefinition {
        name: entry_def_name.clone(),
        fields,
    };

    let mut app_file_tree = add_entry_type_to_integrity_zome(
        app_file_tree,
        &dna_manifest,
        integrity_zome_name,
        &entry_def,
    )?;

    for d in depends_on.iter() {
        app_file_tree = add_link_type_to_integrity_zome(
            app_file_tree,
            &dna_manifest,
            integrity_zome_name,
            &link_type_name(&d, &entry_def_name),
        )?;
    }

    let coordinator_zomes_for_integrity =
        get_coordinator_zomes_for_integrity(&dna_manifest, integrity_zome_name);

    let coordinator_zome = match coordinator_zomes_for_integrity.len() {
        0 => Err(ScaffoldError::NoCoordinatorZomesFoundForIntegrityZome(
            dna_manifest.name(),
            integrity_zome_name.clone(),
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

    let crud = match maybe_crud {
        Some(c) => c.clone(),
        None => choose_crud(),
    };
    let app_file_tree = add_crud_functions_to_coordinator(
        app_file_tree,
        &dna_manifest,
        integrity_zome_name,
        &coordinator_zome,
        entry_def_name,
        &crud,
        &depends_on,
    )?;

    let mut create_fns_for_depends_on: BTreeMap<String, (ZomeManifest, String)> = BTreeMap::new();

    for d in depends_on.clone() {
        let (zome, fn_name) = find_extern_function_or_choose(
            &app_file_tree,
            &dna_manifest,
            &coordinator_zomes_for_integrity,
            &format!("create_{}", d.to_case(Case::Snake)),
            &format!("In which function is a {} created", d.to_case(Case::Pascal)),
        )?;

        create_fns_for_depends_on.insert(d.clone(), (zome, fn_name));
    }

    let app_file_tree = add_tryorama_tests_for_entry_def(
        app_file_tree,
        app_manifest_path,
        &dna_manifest.name(),
        &coordinator_zome.name.0.to_string(),
        &entry_def,
        &crud,
        &create_fns_for_depends_on,
    )?;

    let app_file_tree = scaffold_entry_type_templates(
        app_file_tree,
        &dna_manifest.name(),
        &coordinator_zome.name.0.to_string(),
        &entry_def,
        &depends_on,
    )?;

    Ok(app_file_tree)
}

fn choose_crud() -> Crud {
    let selections = MultiSelect::with_theme(&ColorfulTheme::default())
        .with_prompt("Which CRUD functions should be scaffolded (SPACE to select/unselect, ENTER to continue)?")
        .item_checked("Read", true)
        .item_checked("Update", true)
        .item_checked("Delete", true)
        .interact()
        .unwrap();

    let mut crud = Crud {
        delete: false,
        read: false,
        update: false,
    };

    for selection in selections {
        if selection == 0 {
            crud.read = true;
        }
        if selection == 1 {
            crud.update = true;
        }
        if selection == 2 {
            crud.delete = true;
        }
    }

    crud
}
