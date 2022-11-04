use std::{collections::BTreeMap, path::PathBuf};

use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};

use crate::{
    definitions::{FieldDefinition, FieldType, FieldWidget},
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{dir_content, FileTree},
};

pub fn choose_widget(
    field_type: &FieldType,
    vector: bool,
    field_types_templates: &FileTree,
) -> ScaffoldResult<Option<FieldWidget>> {
    let path = PathBuf::new().join(field_type.to_string());

    match dir_content(field_types_templates, &path) {
        Err(_) => Ok(None),
        Ok(folders) => {
            let widgets_that_can_render_this_type: Vec<String> = folders
                .keys()
                .map(|s| s.to_str().unwrap().to_string())
                .collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Choose widget to render this field:")
                .default(0)
                .items(&widgets_that_can_render_this_type[..])
                .interact()?;

            let widget_name = widgets_that_can_render_this_type[selection].clone();
            let properties: BTreeMap<String, String> = BTreeMap::new();
            // TODO: ask for the properties
            Ok(Some(FieldWidget {
                widget_name,
                properties,
            }))
        }
    }
}

pub fn choose_field(field_types_templates: &FileTree) -> ScaffoldResult<(String, FieldDefinition)> {
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
        .item("Vector of...")
        .interact()?;

    // If user selected vector
    let (vector, field_type) = if selection == field_type_names.len() {
        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Vector of which field type?")
            .default(0)
            .items(&field_type_names[..])
            .interact()?;

        (true, field_types[selection].clone())
    } else {
        (false, field_types[selection].clone())
    };

    let visible = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt("Should this field be visible in the UI?")
        .interact()?;

    let widget = match visible {
        false => None,
        true => choose_widget(&field_type, vector, field_types_templates)?,
    };

    Ok((
        field_name,
        FieldDefinition {
            widget,
            vector,
            field_type,
        },
    ))
}

pub fn choose_fields(
    field_types_templates: &FileTree,
    mut initial_fields: Vec<(String, FieldDefinition)>,
) -> ScaffoldResult<Vec<(String, FieldDefinition)>> {
    let mut finished = false;
    if initial_fields.len() > 0 {
        println!(
            "\nThe entry already contains these fields: {}\n",
            initial_fields
                .iter()
                .map(|f| f.0.clone())
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
        let (field_name, field_def) = choose_field(field_types_templates)?;
        println!("");

        initial_fields.push((field_name, field_def));
        finished = !Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Add another field to the entry?")
            .report(false)
            .interact()?;
    }

    println!(
        "Chosen fields: {}",
        initial_fields
            .iter()
            .map(|f| f.0.clone())
            .collect::<Vec<String>>()
            .join(", ")
    );

    Ok(initial_fields)
}
