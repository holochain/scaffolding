use std::path::PathBuf;

use convert_case::{Case, Casing};
use dialoguer::{theme::ColorfulTheme, Confirm, Select};
use regex::Regex;

use crate::{
    error::ScaffoldResult,
    file_tree::{dir_content, FileTree},
    scaffold::zome::ZomeFileTree,
    utils::{check_case, input_with_case, input_with_case_and_initial_text},
};

use super::{
    definitions::{Cardinality, EntryTypeReference, FieldDefinition, FieldType, Referenceable},
    integrity::get_all_entry_types,
};

fn parse_enum(fields_str: &str) -> ScaffoldResult<FieldType> {
    let sp: Vec<&str> = fields_str.split(":").collect();

    let label = sp[3].to_string().to_case(Case::Pascal);

    let variants = sp[4]
        .to_string()
        .split(".")
        .map(|v| v.to_case(Case::Pascal))
        .collect();

    Ok(FieldType::Enum { label, variants })
}

pub fn parse_fields(fields_str: &str) -> ScaffoldResult<FieldDefinition> {
    let sp: Vec<&str> = fields_str.split(":").collect();

    let field_name = sp[0].to_string();

    check_case(&field_name, "field_name", Case::Snake)?;

    let field_type_str = sp[1].to_string();

    let vec_regex = Regex::new(r"Vec<(?P<a>(.)*)>\z").unwrap();
    let option_regex = Regex::new(r"Option<(?P<a>(.)*)>\z").unwrap();

    let (field_type, cardinality) = if vec_regex.is_match(field_type_str.as_str()) {
        let field_type = vec_regex.replace(field_type_str.as_str(), "${a}");

        if field_type == "Enum" {
            (parse_enum(fields_str)?, Cardinality::Vector)
        } else {
            (
                FieldType::try_from(field_type.to_string())?,
                Cardinality::Vector,
            )
        }
    } else if option_regex.is_match(field_type_str.as_str()) {
        let field_type = option_regex.replace(field_type_str.as_str(), "${a}");

        if field_type == "Enum" {
            (parse_enum(fields_str)?, Cardinality::Option)
        } else {
            (
                FieldType::try_from(field_type.to_string())?,
                Cardinality::Option,
            )
        }
    } else {
        if field_type_str == "Enum" {
            (parse_enum(fields_str)?, Cardinality::Single)
        } else {
            (
                FieldType::try_from(field_type_str.to_string())?,
                Cardinality::Single,
            )
        }
    };

    let widget = if sp.len() > 2 {
        match sp[2] {
            "" => None,
            _ => Some(sp[2].to_string()),
        }
    } else {
        None
    };

    let linked_from = match field_type {
        FieldType::AgentPubKey => match sp.len() {
            4 => Some(Referenceable::Agent {
                role: sp[3].to_string(),
            }),
            _ => None,
        },
        FieldType::EntryHash | FieldType::ActionHash => match sp.len() {
            4 => Some(Referenceable::EntryType(EntryTypeReference {
                entry_type: sp[3].to_string(),
                reference_entry_hash: match field_type {
                    FieldType::EntryHash => true,
                    _ => false,
                },
            })),
            _ => None,
        },
        _ => None,
    };

    Ok(FieldDefinition {
        field_name,
        field_type,
        widget,
        cardinality,
        linked_from,
    })
}

pub fn choose_widget(
    field_type: &FieldType,
    field_types_templates: &FileTree,
) -> ScaffoldResult<Option<String>> {
    let path = PathBuf::new().join(field_type.to_string());

    match dir_content(field_types_templates, &path) {
        Err(_) => Ok(None),
        Ok(folders) => {
            let widgets_that_can_render_this_type: Vec<String> = folders
                .into_iter()
                .filter(|(_key, value)| value.dir_content().is_some())
                .map(|(key, _value)| key)
                .map(|s| s.to_str().unwrap().to_string())
                .collect();

            if widgets_that_can_render_this_type.len() == 0 {
                return Ok(None);
            }

            let visible = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Should this field be visible in the UI?")
                .interact()?;

            if !visible {
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
    }
}

pub fn choose_field(
    entry_type_name: &String,
    zome_file_tree: &ZomeFileTree,
    field_types_templates: &FileTree,
) -> ScaffoldResult<FieldDefinition> {
    let field_types = FieldType::list();
    let field_type_names: Vec<String> = field_types
        .clone()
        .into_iter()
        .map(|s| s.to_string())
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose field type:")
        .default(0)
        .items(&field_type_names[..])
        .item("Option of...")
        .item("Vector of...")
        .interact()?;

    // If user selected vector
    let (cardinality, mut field_type) = if selection == field_type_names.len() {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Option of which field type?")
            .default(0)
            .items(&field_type_names[..])
            .interact()?;

        (Cardinality::Option, field_types[selection].clone())
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
        let label = input_with_case(&String::from("Enter the name of the enum:"), Case::Pascal)?;

        let mut variants: Vec<String> = Vec::new();

        let mut another_field = true;

        while another_field {
            let variant = input_with_case(
                &String::from("Enter the name of the next variant:"),
                Case::Pascal,
            )?;
            variants.push(variant);
            another_field = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Add another variant to the enum?")
                .report(false)
                .interact()?;
        }

        field_type = FieldType::Enum { label, variants };
    }

    let maybe_linked_from = match &field_type {
        FieldType::AgentPubKey => {
            let link_from = Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt(
                    "Should a link from the AgentPubKey provided in this field also be created when entries of this type are created?"
                )
                .interact()?;

            match link_from {
                false => None,
                true => {
                    let role = input_with_case(&String::from(
                        "Which role does this agent play in the relationship ? (eg. \"creator\", \"invitee\")",
                    ), Case::Snake)?;
                    Some(Referenceable::Agent { role })
                }
            }
        }
        FieldType::ActionHash | FieldType::EntryHash => {
            let all_entry_types = get_all_entry_types(zome_file_tree)?.unwrap_or(vec![]);
            if all_entry_types.len() == 0 {
                None
            } else {
                let link_from = Confirm::with_theme(&ColorfulTheme::default())
                    .with_prompt(
                        format!(
                            "Should a link from the {} provided in this field also be created when entries of this type are created?", 
                            field_type.to_string())
                    )
                    .interact()?;

                match link_from {
                    false => None,
                    true => {
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

                        let selection = Select::with_theme(&ColorfulTheme::default())
                            .with_prompt(String::from(
                                "Which entry type is this field referring to?",
                            ))
                            .default(0)
                            .items(&all_options[..])
                            .interact()?;

                        let reference_entry_hash = match field_type {
                            FieldType::EntryHash => true,
                            _ => false,
                        };

                        match selection == all_entry_types.len() {
                            true => Some(Referenceable::EntryType(EntryTypeReference {
                                entry_type: entry_type_name.clone(),
                                reference_entry_hash,
                            })),
                            false => Some(Referenceable::EntryType(EntryTypeReference {
                                entry_type: all_entry_types[selection].entry_type.clone(),
                                reference_entry_hash,
                            })),
                        }
                    }
                }
            }
        }
        _ => None,
    };

    let initial_text = match &maybe_linked_from {
        Some(r) => r.field_name(&cardinality),
        None => String::from(""),
    };

    let field_name: String =
        input_with_case_and_initial_text(&String::from("Field name:"), Case::Snake, &initial_text)?;

    let widget = choose_widget(&field_type, field_types_templates)?;

    Ok(FieldDefinition {
        widget,
        field_name,
        cardinality,
        field_type,
        linked_from: maybe_linked_from,
    })
}

pub fn choose_fields(
    entry_type_name: &String,
    zome_file_tree: &ZomeFileTree,
    field_types_templates: &FileTree,
) -> ScaffoldResult<Vec<FieldDefinition>> {
    let mut finished = false;
    let mut fields: Vec<FieldDefinition> = Vec::new();
    println!("\nWhich fields should the entry contain?\n");

    while !finished {
        let field_def = choose_field(entry_type_name, zome_file_tree, field_types_templates)?;
        println!("");

        fields.push(field_def);
        finished = !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Add another field to the entry?")
            .report(false)
            .interact()?;
    }

    println!(
        "Chosen fields: {}
",
        fields
            .iter()
            .map(|f| f.field_name.clone())
            .collect::<Vec<String>>()
            .join(", ")
    );

    Ok(fields)
}
