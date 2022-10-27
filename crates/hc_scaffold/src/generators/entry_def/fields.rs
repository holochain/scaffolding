use std::collections::BTreeMap;

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use quote::quote;

use crate::{
    definitions::{EntryDefinition, FieldDefinition, FieldType},
    error::{ScaffoldError, ScaffoldResult},
    generators::entry_def::integrity::render_entry_definition_struct,
};

fn list_names() -> Vec<String> {
    vec![
        "TextField",
        "TextArea",
        // "DateAndTime",
        // "Date",
        "Slider",
        "Checkbox",
        // "Switch",
        "AgentPubKey",
        "ActionHash",
        "EntryHash",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

fn from_name(name: &String) -> ScaffoldResult<FieldType> {
    match name.as_str() {
        "TextField" => Ok(FieldType::TextField),
        "TextArea" => Ok(FieldType::TextArea),
        "DateAndTime" => Ok(FieldType::DateAndTime),
        "Date" => Ok(FieldType::Date),
        "Slider" => Ok(FieldType::Slider { min: 0, max: 10 }),
        // "RadioButton" => Ok(Widget::RadioButton {
        //     label: String::from(""),
        //     options: vec![],
        // }),
        "Checkbox" => Ok(FieldType::Checkbox),
        "Switch" => Ok(FieldType::Switch),
        "ActionHash" => Ok(FieldType::ActionHash),
        "EntryHash" => Ok(FieldType::EntryHash),
        "AgentPubKey" => Ok(FieldType::AgentPubKey),
        _ => Err(ScaffoldError::InvalidFieldType(
            name.clone(),
            list_names().join(", "),
        )),
    }
}

/// This function offers a dialoguer to the user to further configure the field type
pub fn choose_from_name(name: &String) -> ScaffoldResult<FieldType> {
    // TODO: actually implement this
    from_name(name)
}

pub fn choose_field() -> ScaffoldResult<(String, FieldDefinition)> {
    let field_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Field name:")
        .report(false)
        .interact_text()?;

    let field_type_names = list_names();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose field type:")
        .default(0)
        .items(&field_type_names[..])
        .item("Vector")
        .report(false)
        .interact()?;

    // If user selected vector
    if selection == field_type_names.len() {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Vector of which field type?")
            .default(0)
            .items(&field_type_names[..])
            .report(false)
            .interact()?;

        let field_type = choose_from_name(&field_type_names[selection])?;

        Ok((
            field_name,
            FieldDefinition {
                label: String::from(""),
                field_type,
                vector: true,
            },
        ))
    } else {
        let field_type = choose_from_name(&field_type_names[selection])?;

        Ok((
            field_name,
            FieldDefinition {
                label: String::from(""),
                field_type,
                vector: false,
            },
        ))
    }
}

pub fn choose_fields(
    mut initial_fields: BTreeMap<String, FieldDefinition>,
) -> ScaffoldResult<BTreeMap<String, FieldDefinition>> {
    println!("\nWhich fields should the entry contain?\n");

    let mut finished = false;

    if initial_fields.len() > 0 {
        finished = !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Add another field to the entry? Current fields: {}",
                initial_fields
                    .keys()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(", ")
            ))
            .report(false)
            .interact()?;
    }

    while !finished {
        let (field_name, field_type) = choose_field()?;

        initial_fields.insert(field_name, field_type);
        finished = !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Add another field to the entry? Current fields: {}",
                initial_fields
                    .keys()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(", ")
            ))
            .report(false)
            .interact()?;
    }

    println!(
        "Chosen fields: {}",
        initial_fields
            .keys()
            .cloned()
            .collect::<Vec<String>>()
            .join(", ")
    );

    Ok(initial_fields)
}
