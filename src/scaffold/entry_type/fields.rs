use std::path::PathBuf;

use colored::Colorize;
use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{dir_content, FileTree},
    reserved_words::check_for_reserved_keywords,
    scaffold::zome::ZomeFileTree,
    utils::{check_case, input_with_case, input_with_custom_validation},
};

use super::{
    definitions::{Cardinality, EntryTypeReference, FieldDefinition, FieldType, Referenceable},
    integrity::get_all_entry_types,
};

pub fn choose_fields(
    entry_type_name: &str,
    zome_file_tree: &ZomeFileTree,
    field_types_templates: &FileTree,
    no_ui: bool,
) -> ScaffoldResult<Vec<FieldDefinition>> {
    let mut finished = false;
    let mut fields: Vec<FieldDefinition> = Vec::new();

    println!("\nWhich fields should the entry contain?\n");

    while !finished {
        let field_def = choose_field(
            entry_type_name,
            zome_file_tree,
            field_types_templates,
            no_ui,
            None,
        )?;
        println!();

        fields.push(field_def);
        finished = !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Add another field to the entry?")
            .report(false)
            .interact()?;
    }

    println!(
        "Current fields:\n{}\n",
        fields
            .iter()
            .map(|f| format!(" {}: {}", f.field_name.clone(), f.field_type))
            .collect::<Vec<String>>()
            .join("\n")
            .italic()
    );

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(
            "Do you want to proceed with the current entry type, modify or start again from the beginning?",
        )
        .item("Confirm")
        .item("Modify")
        .item("Restart")
        .default(0)
        .interact()?;

    if selection == 1 {
        loop {
            let action = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("What would you like to do?")
                .items(&["Change Field", "Add Field", "Remove Field", "Done"])
                .interact()?;

            match action {
                0 => {
                    // Change field
                    if !fields.is_empty() {
                        let field_to_change = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select field to change")
                            .items(
                                &fields
                                    .iter()
                                    .map(|f| format!("{}: {}", f.field_name, f.field_type).italic())
                                    .collect::<Vec<_>>(),
                            )
                            .interact()?;

                        let new_field = choose_field(
                            entry_type_name,
                            zome_file_tree,
                            field_types_templates,
                            no_ui,
                            Some(&fields[field_to_change].field_name),
                        )?;
                        fields[field_to_change] = new_field;
                    } else {
                        println!("{}", "No fields left to change".yellow())
                    }
                }
                1 => {
                    // Add field
                    let new_field = choose_field(
                        entry_type_name,
                        zome_file_tree,
                        field_types_templates,
                        no_ui,
                        None,
                    )?;
                    fields.push(new_field);
                }
                2 => {
                    // Remove field
                    if !fields.is_empty() {
                        let field_to_remove = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt("Select field to remove")
                            .items(
                                &fields
                                    .iter()
                                    .map(|f| format!("{}: {}", f.field_name, f.field_type).italic())
                                    .collect::<Vec<_>>(),
                            )
                            .interact()?;
                        fields.remove(field_to_remove);
                    } else {
                        println!("{}", "All fields have been removed".yellow())
                    }
                }
                3 => break, // Done
                _ => unreachable!(),
            }

            if !fields.is_empty() {
                println!(
                    "\nCurrent fields:\n{}\n",
                    fields
                        .iter()
                        .map(|f| format!(" {}: {}", f.field_name, f.field_type))
                        .collect::<Vec<String>>()
                        .join("\n")
                        .italic()
                );
            }
        }
    } else if selection == 2 {
        return choose_fields(
            entry_type_name,
            zome_file_tree,
            field_types_templates,
            no_ui,
        );
    }

    Ok(fields)
}

fn choose_field(
    entry_type_name: &str,
    zome_file_tree: &ZomeFileTree,
    field_types_templates: &FileTree,
    no_ui: bool,
    initial_field_name: Option<&str>,
) -> ScaffoldResult<FieldDefinition> {
    let field_types = FieldType::list();
    let field_type_names: Vec<String> = field_types
        .clone()
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let field_name =
        input_with_custom_validation("Field name (snake_case):", initial_field_name, |input| {
            if let Err(e) = check_case(&input, "field_name", Case::Snake) {
                return Err(e.to_string());
            }
            if let Err(e) = check_for_reserved_keywords(&input) {
                return Err(e.to_string());
            }
            Ok(())
        })?;

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose field type:")
        .default(0)
        .items(&field_type_names[..])
        .item("Option of...")
        .item("Vector of...")
        .interact()?;

    // If user selected Vector of ...
    let (cardinality, field_type) = if selection == field_type_names.len() {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Option of which field type?")
            .default(0)
            .items(&field_type_names[..])
            .interact()?;

        (Cardinality::Option, field_types[selection].clone())
    // If user selected Option of ...
    } else if selection == field_type_names.len() + 1 {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Vector of which field type?")
            .default(0)
            .items(&field_type_names[..])
            .interact()?;

        (Cardinality::Vector, field_types[selection].clone())
    } else {
        (Cardinality::Single, field_types[selection].clone())
    };

    if let FieldType::Enum { .. } = field_type {
        let label = input_with_custom_validation(
            "Enter the name of the enum (PascalCase):",
            None,
            |input: String| {
                if !input.is_case(Case::Pascal) {
                    return Err(format!("Input must be {:?} case.", Case::Pascal));
                }
                if input.to_ascii_lowercase() == entry_type_name {
                    return Err(format!(
                        "Enum name: {input} conflicts with entry-type name: {entry_type_name}"
                    ));
                }
                Ok(())
            },
        )?;

        let mut variants = Vec::new();
        let mut another_variant = true;

        while another_variant {
            let variant = input_with_custom_validation(
                "Enter the name of the next variant (PascalCase):",
                None,
                |input: String| {
                    if !input.is_case(Case::Pascal) {
                        return Err(format!("Input must be {:?} case.", Case::Pascal));
                    }
                    if variants.contains(&input) {
                        return Err(format!("{input} is already a variant of the enum"));
                    }
                    Ok(())
                },
            )?;
            variants.push(variant);
            another_variant = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Add another variant to the enum?")
                .report(false)
                .interact()?;
        }

        let widget = (!no_ui)
            .then(|| choose_widget(&field_type, field_types_templates))
            .transpose()?
            .flatten();

        return FieldDefinition::new(
            label.to_case(Case::Snake),
            FieldType::Enum { label, variants },
            widget,
            cardinality,
            None,
        );
    }

    let linked_from = match &field_type {
        FieldType::AgentPubKey => {
            let should_link_from_agent_pubkey = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(
                    "Should a link from the AgentPubKey provided in this field also be created when entries of this type are created?"
                )
                .interact()?;

            if should_link_from_agent_pubkey {
                let role = input_with_case(
                    "Which role does this agent play in the relationship ? (eg. \"creator\", \"invitee\")",
                    None,
                    Case::Snake
                )?;
                Some(Referenceable::Agent { role })
            } else {
                None
            }
        }
        FieldType::ActionHash | FieldType::EntryHash => {
            let should_link_from_hash_type = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(
                        format!(
                            "Should a link from the {field_type} provided in this field also be created when entries of this type are created?",
                        )
                    )
                    .interact()?;

            if should_link_from_hash_type {
                let all_entry_types = get_all_entry_types(zome_file_tree)?.unwrap_or_default();
                let mut all_options: Vec<String> = all_entry_types
                    .clone()
                    .into_iter()
                    .map(|r| r.entry_type)
                    .collect();

                if let Cardinality::Option | Cardinality::Vector = cardinality {
                    all_options.push(format!(
                        "{} (itself)",
                        entry_type_name.to_case(Case::Pascal)
                    ));
                }

                if all_options.is_empty() {
                    return Err(ScaffoldError::NoEntryTypesDefFoundForIntegrityZome(
                        zome_file_tree.dna_file_tree.dna_manifest.name(),
                        zome_file_tree.zome_manifest.name.to_string(),
                    ));
                }

                let selection = Select::with_theme(&ColorfulTheme::default())
                    .with_prompt(String::from("Which entry type is this field referring to?"))
                    .default(0)
                    .items(&all_options[..])
                    .interact()?;

                let reference_entry_hash = matches!(field_type, FieldType::EntryHash);

                // if the entry links to itself
                if selection == all_entry_types.len() {
                    Some(Referenceable::EntryType(EntryTypeReference {
                        entry_type: entry_type_name.to_owned(),
                        reference_entry_hash,
                    }))
                } else {
                    Some(Referenceable::EntryType(EntryTypeReference {
                        entry_type: all_entry_types[selection].entry_type.clone(),
                        reference_entry_hash,
                    }))
                }
            } else {
                None
            }
        }
        _ => None,
    };

    let widget = (!no_ui)
        .then(|| choose_widget(&field_type, field_types_templates))
        .transpose()?
        .flatten();

    FieldDefinition::new(field_name, field_type, widget, cardinality, linked_from)
}

fn choose_widget(
    field_type: &FieldType,
    field_types_templates: &FileTree,
) -> ScaffoldResult<Option<String>> {
    let path = PathBuf::new().join(field_type.to_string());

    match dir_content(field_types_templates, &path) {
        Ok(folders) => {
            let widgets_that_can_render_this_type: Vec<String> = folders
                .into_iter()
                .filter(|(_key, value)| value.dir_content().is_some())
                .map(|(key, _value)| key)
                .map(|s| s.to_str().unwrap().to_string())
                .collect();

            if widgets_that_can_render_this_type.is_empty() {
                return Ok(None);
            }

            let should_scaffold_ui = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Should UI be generated for this field?")
                .interact()?;

            if !should_scaffold_ui {
                return Ok(None);
            }

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose widget to render this field:")
                .default(0)
                .items(&widgets_that_can_render_this_type[..])
                .interact()?;

            let widget_name = widgets_that_can_render_this_type[selection].clone();

            Ok(Some(widget_name))
        }
        Err(_) => Ok(None),
    }
}
