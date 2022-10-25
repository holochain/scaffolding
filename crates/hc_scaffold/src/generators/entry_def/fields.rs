use std::collections::BTreeMap;

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use quote::quote;

use crate::{
    definitions::{EntryDefinition, FieldRepresentation, FieldType, Widget},
    error::{ScaffoldError, ScaffoldResult},
    generators::entry_def::integrity::render_entry_definition_struct,
};

fn list_names() -> Vec<String> {
    vec![
        "TextField",
        "TextArea",
        "DateAndTime",
        "Date",
        "Time",
        "Slider",
        "RadioButton",
        "Checkbox",
        "Switch",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect()
}

fn from_name(name: &String) -> ScaffoldResult<Widget> {
    match name.as_str() {
        "TextField" => Ok(Widget::TextField {
            label: String::from(""),
        }),
        "TextArea" => Ok(Widget::TextArea {
            label: String::from(""),
        }),
        "DateAndTime" => Ok(Widget::DateAndTime {
            label: String::from(""),
        }),
        "Date" => Ok(Widget::Date {
            label: String::from(""),
        }),
        "Time" => Ok(Widget::Time {
            label: String::from(""),
        }),
        "Slider" => Ok(Widget::Slider {
            label: String::from(""),
            min: 0,
            max: 10,
        }),
        "RadioButton" => Ok(Widget::RadioButton {
            label: String::from(""),
            options: vec![],
        }),
        "Checkbox" => Ok(Widget::Checkbox {
            label: String::from(""),
        }),
        "Switch" => Ok(Widget::Switch {
            label: String::from(""),
        }),
        _ => Err(ScaffoldError::InvalidFieldType(
            name.clone(),
            list_names().join(", "),
        )),
    }
}

/// This function offers a dialoguer to the user to further configure the field type
pub fn choose_from_name(name: &String) -> ScaffoldResult<Widget> {
    // TODO: actually implement this
    from_name(name)
}

pub fn choose_field() -> ScaffoldResult<(String, FieldType)> {
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

        let field_widget = choose_from_name(&field_type_names[selection])?;

        Ok((
            field_name,
            FieldType::new_vector(FieldRepresentation::Visible(field_widget)),
        ))
    } else {
        let field_widget = choose_from_name(&field_type_names[selection])?;

        Ok((
            field_name,
            FieldType::new_single(FieldRepresentation::Visible(field_widget)),
        ))
    }
}

pub fn choose_fields(
    mut initial_fields: BTreeMap<String, FieldType>,
) -> ScaffoldResult<BTreeMap<String, FieldType>> {
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
