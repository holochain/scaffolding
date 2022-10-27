use build_fs_tree::{file, serde::Serialize};
use convert_case::{Case, Casing};
use handlebars::{handlebars_helper, Context, Handlebars};
use include_dir::{include_dir, Dir};
use regex::Regex;
use serde_json::Value;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::definitions::FieldType;
use crate::error::{ScaffoldError, ScaffoldResult};
use crate::file_tree::{
    create_dir_all, find_files, find_map_files, flatten_file_tree, map_all_files,
    unflatten_file_tree, FileTree,
};

pub fn register_concat_helper<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    handlebars_helper!(concat: |s1: String, s2: String| format!("{}{}", s1, s2));
    h.register_helper("concat", Box::new(concat));

    h
}

pub fn register_case_helpers<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    handlebars_helper!(title_case: |s: String| s.to_case(Case::Title));
    h.register_helper("title_case", Box::new(title_case));

    handlebars_helper!(snake_case: |s: String| s.to_case(Case::Snake));
    h.register_helper("snake_case", Box::new(snake_case));

    handlebars_helper!(kebab_case: |s: String| s.to_case(Case::Kebab));
    h.register_helper("kebab_case", Box::new(kebab_case));

    handlebars_helper!(camel_case: |s: String| s.to_case(Case::Camel));
    h.register_helper("camel_case", Box::new(camel_case));

    handlebars_helper!(pascal_case: |s: String| s.to_case(Case::Pascal));
    h.register_helper("pascal_case", Box::new(pascal_case));

    h
}

pub fn register_all_partials_in_dir<'a>(
    mut h: Handlebars<'a>,
    file_tree: &FileTree,
) -> ScaffoldResult<Handlebars<'a>> {
    let partials = find_files(file_tree, &|path, contents| {
        if let Some(e) = PathBuf::from(path).extension() {
            if e == "hbs" {
                return true;
            }
        }
        return false;
    });

    for (path, content) in partials {
        h.register_partial(
            path.with_extension("").as_os_str().to_str().unwrap(),
            content,
        )?;
    }

    Ok(h)
}

pub fn render_template_file_tree<'a, T: Serialize>(
    h: &Handlebars<'a>,
    templates_file_tree: &FileTree,
    data: &T,
) -> ScaffoldResult<FileTree> {
    let mut file_tree = templates_file_tree.clone();

    let flattened_templates = flatten_file_tree(templates_file_tree);

    let mut transformed_templates: BTreeMap<PathBuf, String> = BTreeMap::new();

    for (path, contents) in flattened_templates {
        if let Some(e) = path.extension() {
            let re = Regex::new(r"\A(?P<a>(.)*).hbs.\{\{#each (?P<b>(.)*)\}\}\z").unwrap();

            if re.is_match(path.to_str().unwrap()) {
                let new_path =
                    re.replace(path.to_str().unwrap(), "{{#each ${b} }}${a}.hbs{{/each}}");
                let all_paths = h.render_template(new_path.to_string().as_str(), data)?;

                let files_to_create: Vec<String> =
                    all_paths.split(".hbs").map(|s| s.to_string()).collect();

                for f in files_to_create {
                    let new_contents = h.render_template(contents.as_str(), data)?;
                    transformed_templates.insert(PathBuf::from(f).with_extension(""), new_contents);
                }
            } else if e == "hbs" {
                let new_path = h.render_template(path.as_os_str().to_str().unwrap(), data)?;
                let new_contents = h.render_template(contents.as_str(), data)?;

                transformed_templates
                    .insert(PathBuf::from(new_path).with_extension(""), new_contents);
            }
        }
    }

    unflatten_file_tree(&transformed_templates)
}

pub fn render_template_file_tree_and_merge_with_existing<'a, T: Serialize>(
    app_file_tree: FileTree,
    h: &Handlebars<'a>,
    template_file_tree: &FileTree,
    data: &T,
) -> ScaffoldResult<FileTree> {
    let rendered_templates = render_template_file_tree(h, template_file_tree, data)?;

    let mut flattened_app_file_tree = flatten_file_tree(&app_file_tree);
    let flattened_templates = flatten_file_tree(&rendered_templates);

    flattened_app_file_tree.extend(flattened_templates);

    unflatten_file_tree(&flattened_app_file_tree)
}
