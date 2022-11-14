use std::{collections::BTreeMap, path::PathBuf};

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{dir_content, FileTree},
};

use super::definitions::{Cardinality, FieldDefinition, FieldType};

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

pub fn choose_field(field_types_templates: &FileTree) -> ScaffoldResult<FieldDefinition> {
    let field_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Field name:")
        .interact_text()?;

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
    let (cardinality, field_type) = if selection == field_type_names.len() {
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

    let widget = choose_widget(&field_type, field_types_templates)?;

    Ok(FieldDefinition {
        widget,
        field_name,
        cardinality,
        field_type,
    })
}

pub fn choose_fields(
    field_types_templates: &FileTree,
    mut initial_fields: Vec<FieldDefinition>,
) -> ScaffoldResult<Vec<FieldDefinition>> {
    let mut finished = false;
    if initial_fields.len() > 0 {
        println!(
            "\nThe entry already contains these fields: {}\n",
            initial_fields
                .iter()
                .map(|f| f.field_name.clone())
                .collect::<Vec<String>>()
                .join(", ")
        );
        finished = !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Add another field to the entry?")
            .report(false)
            .interact()?;
    } else {
        println!("\nWhich fields should the entry contain?\n");
    }

    while !finished {
        let field_def = choose_field(field_types_templates)?;
        println!("");

        initial_fields.push(field_def);
        finished = !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Add another field to the entry?")
            .report(false)
            .interact()?;
    }

    println!(
        "Chosen fields: {}
",
        initial_fields
            .iter()
            .map(|f| f.field_name.clone())
            .collect::<Vec<String>>()
            .join(", ")
    );

    Ok(initial_fields)
}
