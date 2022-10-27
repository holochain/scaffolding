use std::{collections::BTreeMap, ffi::OsString, path::PathBuf};

use crate::{
    definitions::{EntryDefinition, FieldDefinition, FieldType},
    file_tree::FileTree,
};
use build_fs_tree::file;
use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Confirm, Input, MultiSelect, Select};
use holochain_types::prelude::{AppManifest, DnaManifest, ZomeManifest};

use crate::{
    cli::Crud,
    error::{ScaffoldError, ScaffoldResult},
};

use self::{
    coordinator::add_crud_functions_to_coordinator,
    fields::choose_fields,
    integrity::{add_entry_def_to_integrity_zome, get_all_entry_types},
    utils::choose_multiple_entry_types,
};

use super::{
    link_type::{integrity::add_link_type_to_integrity_zome, link_type_name},
    tryorama::add_tryorama_tests_for_entry_def,
    web_app::uis::scaffold_entry_type_templates,
    zome::{
        coordinator::find_extern_function_or_choose,
        utils::{get_coordinator_zomes_for_integrity, zome_manifest_path},
    },
};

pub mod coordinator;
pub mod fields;
pub mod integrity;
pub mod utils;

fn get_or_choose_depends_on(
    app_file_tree: &FileTree,
    app_manifest: &AppManifest,
    dna_manifest: &DnaManifest,
    integrity_zome_name: &String,
    depends_on: &Option<Vec<String>>,
) -> ScaffoldResult<Vec<String>> {
    let entry_types = get_all_entry_types(
        app_file_tree,
        app_manifest,
        dna_manifest,
        integrity_zome_name,
    )?
    .unwrap_or_else(|| vec![]);

    if entry_types.len() == 0 {
        return Ok(vec![]);
    }

    match depends_on {
        Some(et) => match et.iter().find(|t| !entry_types.contains(t)) {
            Some(t) => Err(ScaffoldError::EntryTypeNotFound(
                t.clone(),
                dna_manifest.name(),
                integrity_zome_name.clone(),
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

pub fn scaffold_entry_def(
    mut app_file_tree: FileTree,
    app_manifest: &(PathBuf, AppManifest),
    dna_manifest: &DnaManifest,
    integrity_zome_name: &String,
    entry_def_name: &String,
    maybe_crud: &Option<Crud>,
    maybe_depends_on: &Option<Vec<String>>,
    maybe_fields: &Option<BTreeMap<String, FieldType>>,
) -> ScaffoldResult<FileTree> {
    let depends_on: Vec<String> = get_or_choose_depends_on(
        &app_file_tree,
        &app_manifest.1,
        dna_manifest,
        integrity_zome_name,
        maybe_depends_on,
    )?;

    let fields = match maybe_fields {
        Some(f) => {
            let mut fields: BTreeMap<String, FieldDefinition> = BTreeMap::new();

            for (field_name, field_type) in f {
                fields.insert(
                    field_name.clone(),
                    FieldDefinition {
                        label: String::from(""),
                        vector: false,
                        field_type: field_type.clone(),
                    },
                );
            }
            for d in depends_on.clone() {
                let field_name = format!("{}_hash", d.to_case(Case::Snake));
                fields.insert(
                    field_name,
                    FieldDefinition {
                        label: String::from(""),
                        vector: false,
                        field_type: FieldType::ActionHash,
                    },
                );
            }
            fields
        }
        None => {
            let mut initial_fields: BTreeMap<String, FieldDefinition> = BTreeMap::new();

            for d in depends_on.clone() {
                let field_name = format!("{}_hash", d.to_case(Case::Snake));
                initial_fields.insert(
                    field_name,
                    FieldDefinition {
                        label: String::from(""),
                        vector: false,
                        field_type: FieldType::ActionHash,
                    },
                );
            }
            choose_fields(initial_fields)?
        }
    };

    let entry_def = EntryDefinition {
        name: entry_def_name.clone(),
        fields,
    };

    let mut app_file_tree = add_entry_def_to_integrity_zome(
        app_file_tree,
        &app_manifest.1,
        dna_manifest,
        integrity_zome_name,
        &entry_def,
    )?;

    for d in depends_on.iter() {
        app_file_tree = add_link_type_to_integrity_zome(
            app_file_tree,
            dna_manifest,
            integrity_zome_name,
            &link_type_name(&d, &entry_def_name),
        )?;
    }

    let coordinator_zomes_for_integrity =
        get_coordinator_zomes_for_integrity(dna_manifest, integrity_zome_name);

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
        dna_manifest,
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
            dna_manifest,
            &coordinator_zomes_for_integrity,
            &format!("create_{}", d.to_case(Case::Snake)),
            &format!("In which function is a {} created", d.to_case(Case::Pascal)),
        )?;

        create_fns_for_depends_on.insert(d.clone(), (zome, fn_name));
    }

    let app_file_tree = add_tryorama_tests_for_entry_def(
        app_file_tree,
        app_manifest,
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
