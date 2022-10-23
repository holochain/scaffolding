use std::collections::BTreeMap;

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

use crate::{
    definitions::{FieldRepresentation, FieldType, Widget},
    error::{ScaffoldError, ScaffoldResult},
};

fn list_names() -> Vec<String> {
    vec![
        "TextArea",
        "TextField",
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
        .interact_text()
        .unwrap();

    let field_type_names = list_names();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose field type:")
        .default(0)
        .items(&field_type_names[..])
        .item("Vector")
        .interact()?;

    // If user selected vector
    if selection == field_type_names.len() {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Vector of which field type?")
            .default(0)
            .items(&field_type_names[..])
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

pub fn choose_fields() -> ScaffoldResult<BTreeMap<String, FieldType>> {
    println!("Which fields should the entry contain?");

    let mut fields: BTreeMap<String, FieldType> = BTreeMap::new();

    let mut finished = false;

    while !finished {
        let (field_name, field_type) = choose_field()?;

        fields.insert(field_name, field_type);

        finished = !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Add another field to the entry?")
            .interact()?;
    }

    Ok(fields)
}
