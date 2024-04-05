use std::{ffi::OsString, path::PathBuf};

use crate::{
    file_tree::FileTree,
    reserved_words::check_for_reserved_words,
    templates::{entry_type::scaffold_entry_type_templates, ScaffoldedTemplate},
};

use build_fs_tree::dir;
use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};

use crate::error::{ScaffoldError, ScaffoldResult};

use self::{
    coordinator::{add_crud_functions_to_coordinator, updates_link_name},
    crud::Crud,
    definitions::{EntryDefinition, EntryTypeReference, FieldDefinition, Referenceable},
    fields::choose_fields,
    integrity::{add_entry_type_to_integrity_zome, get_all_entry_types},
};

use super::{
    app::AppFileTree,
    link_type::{integrity::add_link_type_to_integrity_zome, link_type_name},
    zome::{utils::get_coordinator_zomes_for_integrity, ZomeFileTree},
};

pub mod coordinator;
pub mod crud;
pub mod definitions;
pub mod fields;
pub mod integrity;
pub mod utils;

fn check_field_definitions(
    entry_type_name: &str,
    zome_file_tree: &ZomeFileTree,
    fields: &[FieldDefinition],
) -> ScaffoldResult<()> {
    let entry_types = get_all_entry_types(zome_file_tree)?.unwrap_or_else(Vec::new);

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
        !entry_types_names.contains(&l.entry_type.to_case(Case::Pascal))
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

// TODO: group some params into a new-type or prefer builder pattern
#[allow(clippy::too_many_arguments)]
pub fn scaffold_entry_type(
    zome_file_tree: ZomeFileTree,
    template_file_tree: &FileTree,
    name: &str,
    maybe_crud: &Option<Crud>,
    maybe_reference_entry_hash: Option<bool>,
    maybe_link_from_original_to_each_update: Option<bool>,
    maybe_fields: Option<&Vec<FieldDefinition>>,
    no_ui: bool,
) -> ScaffoldResult<ScaffoldedTemplate> {
    check_for_reserved_words(name)?;

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
                no_ui,
            )?
        }
    };

    let reference_entry_hash = maybe_reference_entry_hash.unwrap_or(false);

    let crud = match maybe_crud {
        Some(c) => c.clone(),
        None => choose_crud(),
    };

    let link_from_original_to_each_update = match crud.update {
        true => match maybe_link_from_original_to_each_update {
            Some(l) => l,
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
        name: name.to_owned(),
        fields,
        reference_entry_hash,
    };

    let integrity_zome_name = zome_file_tree.zome_manifest.name.0.to_string();

    let mut zome_file_tree = add_entry_type_to_integrity_zome(zome_file_tree, &entry_def, &crud)?;

    let linked_from: Vec<Referenceable> = entry_def
        .fields
        .iter()
        .filter_map(|f| f.linked_from.clone())
        .collect();

    for l in linked_from.clone() {
        zome_file_tree = add_link_type_to_integrity_zome(
            zome_file_tree,
            &link_type_name(&l, &entry_def.referenceable()),
            &Some(l),
            &Some(entry_def.referenceable()),
            crud.delete,
            &PathBuf::from(format!("{}.rs", entry_def.name.to_case(Case::Snake))),
        )?;
    }

    let coordinator_zomes_for_integrity = get_coordinator_zomes_for_integrity(
        &zome_file_tree.dna_file_tree.dna_manifest,
        zome_file_tree.zome_manifest.name.0.as_ref(),
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
        zome_file_tree = add_link_type_to_integrity_zome(
            zome_file_tree,
            &updates_link_name(&entry_def.name),
            &Some(entry_def.referenceable()),
            &Some(entry_def.referenceable()),
            false,
            &PathBuf::from(format!("{}.rs", entry_def.name.to_case(Case::Snake))),
        )?;
    }

    let mut zome_file_tree =
        ZomeFileTree::from_zome_manifest(zome_file_tree.dna_file_tree, coordinator_zome.clone())?;

    zome_file_tree = add_crud_functions_to_coordinator(
        zome_file_tree,
        &integrity_zome_name,
        &entry_def,
        &crud,
        link_from_original_to_each_update,
    )?;

    let dna_manifest = zome_file_tree.dna_file_tree.dna_manifest.clone();

    let app_file_tree = AppFileTree::get_or_choose(zome_file_tree.dna_file_tree.file_tree(), None)?;

    let app_name = app_file_tree.app_manifest.app_name().to_string();

    scaffold_entry_type_templates(
        app_file_tree.file_tree(),
        template_file_tree,
        &app_name,
        &dna_manifest.name(),
        &coordinator_zome,
        &entry_def,
        &crud,
        link_from_original_to_each_update,
        no_ui,
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
