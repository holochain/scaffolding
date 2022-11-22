use std::{collections::BTreeMap, ffi::OsString, path::PathBuf};

use crate::{
    file_tree::FileTree,
    templates::{entry_type::scaffold_entry_type_templates, ScaffoldedTemplate},
};

use build_fs_tree::dir;
use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};
use holochain_types::prelude::ZomeManifest;

use crate::error::{ScaffoldError, ScaffoldResult};

use self::{
    coordinator::{add_crud_functions_to_coordinator, updates_link_name},
    crud::Crud,
    definitions::{EntryDefinition, EntryTypeReference, FieldDefinition, Referenceable},
    fields::choose_fields,
    integrity::{add_entry_type_to_integrity_zome, get_all_entry_types},
};

use super::{
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

fn check_field_definitions(
    entry_type_name: &String,
    zome_file_tree: &ZomeFileTree,
    fields: &Vec<FieldDefinition>,
) -> ScaffoldResult<()> {
    let entry_types = get_all_entry_types(zome_file_tree)?.unwrap_or_else(|| vec![]);

    let entry_types_names: Vec<String> = entry_types
        .clone()
        .into_iter()
        .map(|et| et.entry_type.clone())
        .collect();

    let linked_from_entry_type: Vec<EntryTypeReference> = fields
        .iter()
        .filter_map(|f| f.linked_from.clone())
        .filter_map(|t| match t {
            Referenceable::Agent { .. } => None,
            Referenceable::EntryType(et) => Some(et),
        })
        .collect();

    match linked_from_entry_type.into_iter().find(|l| {
        !entry_types_names.contains(&l.entry_type)
            && !l
                .entry_type
                .to_case(Case::Pascal)
                .eq(&entry_type_name.to_case(Case::Pascal))
    }) {
        Some(t) => Err(ScaffoldError::EntryTypeNotFound(
            t.entry_type.clone(),
            zome_file_tree.dna_file_tree.dna_manifest.name(),
            zome_file_tree.zome_manifest.name.0.to_string(),
        )),
        None => Ok(()),
    }
}

pub fn scaffold_entry_type(
    zome_file_tree: ZomeFileTree,
    template_file_tree: &FileTree,
    name: &String,
    maybe_crud: &Option<Crud>,
    maybe_reference_entry_hash: &Option<bool>,
    maybe_link_from_original_to_each_update: &Option<bool>,
    maybe_fields: &Option<Vec<FieldDefinition>>,
) -> ScaffoldResult<ScaffoldedTemplate> {
    let fields = match maybe_fields {
        Some(f) => {
            check_field_definitions(name, &zome_file_tree, f)?;
            f.clone()
        }
        None => {
            let v: Vec<OsString> = PathBuf::from("field-types")
                .iter()
                .map(|s| s.to_os_string())
                .collect();
            let empty_dir = dir! {};
            choose_fields(
                name,
                &zome_file_tree,
                template_file_tree.path(&mut v.iter()).unwrap_or(&empty_dir),
            )?
        }
    };

    let reference_entry_hash = match maybe_reference_entry_hash {
        Some(r) => r.clone(),
        None => {
            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt(String::from(
                    "Choose hash type to refererence this entry type:",
                ))
                .default(0)
                .item("ActionHash (recommended)")
                .item("EntryHash")
                .interact()?;

            match selection {
                0 => false,
                _ => true,
            }
        }
    };

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

    let entry_def = EntryDefinition {
        name: name.clone(),
        fields,
        reference_entry_hash,
    };

    let integrity_zome_name = zome_file_tree.zome_manifest.name.0.to_string();

    let mut zome_file_tree = add_entry_type_to_integrity_zome(zome_file_tree, &entry_def)?;

    let linked_from: Vec<Referenceable> = entry_def
        .fields
        .iter()
        .filter_map(|f| f.linked_from.clone())
        .collect();

    for l in linked_from.clone() {
        zome_file_tree = add_link_type_to_integrity_zome(
            zome_file_tree,
            &link_type_name(&l, &entry_def.referenceable()),
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

    for l in linked_from.clone() {
        if let Referenceable::EntryType(entry_type_reference) = l {
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
