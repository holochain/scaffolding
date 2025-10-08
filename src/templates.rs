use anyhow::Context;
use build_fs_tree::serde::Serialize;
use handlebars::Handlebars;
use regex::Regex;
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::sync::LazyLock;

use crate::error::{ScaffoldError, ScaffoldResult};
use crate::file_tree::{
    file_content, find_files, flatten_file_tree, unflatten_file_tree, FileTree,
};
use crate::utils::format_code;

pub mod helpers;

pub mod collection;
pub mod coordinator;
pub mod dna;
pub mod entry_type;
pub mod example;
pub mod integrity;
pub mod link_type;
pub mod web_app;

static EACH_TEMPLATE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?P<c>(.)*)/\{\{#each (?P<b>([^\{\}])*)\}\}(?P<a>(.)*)\{\{/each\}\}.hbs\z")
        .expect("EACH_TEMPLATE_REGEX is invalid")
});
static EACH_IF_TEMPLATE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?P<c>(.)*)/\{\{#each (?P<b>([^\{\}])*)\}\}\{\{#if (?P<d>([^\{\}])*)\}\}(?P<a>(.)*)\{\{/if\}\}\{\{/each\}\}.hbs\z")
        .expect("EACH_IF_TEMPLATE_REGEX is invalid")
});
static IF_TEMPLATE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?P<c>(.)*)/\{\{#if (?P<b>([^\{\}])*)\}\}(?P<a>(.)*)\{\{/if\}\}.hbs\z")
        .expect("IF_TEMPLATE_REGEX is invalid")
});

pub struct ScaffoldedTemplate {
    pub file_tree: FileTree,
    pub next_instructions: Option<String>,
}

pub fn build_handlebars<'a>(templates_dir: &FileTree) -> ScaffoldResult<Handlebars<'a>> {
    let h = Handlebars::new();

    let mut h = helpers::register_helpers(h);

    let field_types_path = PathBuf::from("field-types");
    let v: Vec<OsString> = field_types_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(field_types_templates) = templates_dir.path(&mut v.iter()) {
        h = register_all_partials_in_dir(h, field_types_templates)?;
    }
    h.register_escape_fn(handlebars::no_escape);

    Ok(h)
}

pub fn register_all_partials_in_dir<'a>(
    mut h: Handlebars<'a>,
    file_tree: &FileTree,
) -> ScaffoldResult<Handlebars<'a>> {
    let partials = find_files(file_tree, &|path, _contents| {
        PathBuf::from(path).extension().is_some_and(|e| e == "hbs")
    });

    for (path, content) in partials {
        h.register_partial(
            path.with_extension("")
                .as_os_str()
                .to_str()
                .context("Failed to convert OsStr to str")?,
            content.trim(),
        )
        .map_err(Box::new)?;
    }

    Ok(h)
}

pub fn render_template_file(
    h: &Handlebars,
    existing_app_file_tree: &FileTree,
    target_path: &Path,
    template_str: &str,
    value: &serde_json::Value,
) -> ScaffoldResult<String> {
    let mut value = value.clone();

    if let Ok(previous_content) = file_content(existing_app_file_tree, target_path) {
        value
            .as_object_mut()
            .context("Failed to get Value as a mutable object")?
            .insert("previous_file_content".into(), previous_content.into());
    }

    let mut h = h.clone();
    h.register_template_string(target_path.to_str().unwrap(), template_str)
        .map_err(Box::new)?;
    let new_contents = h.render(target_path.to_str().unwrap(), &value)?;

    Ok(new_contents)
}

pub fn render_template_file_tree<T: Serialize>(
    existing_app_file_tree: &FileTree,
    h: &Handlebars,
    templates_file_tree: &FileTree,
    data: &T,
) -> ScaffoldResult<FileTree> {
    let flattened_templates = flatten_file_tree(templates_file_tree);

    let mut transformed_templates: BTreeMap<PathBuf, Option<String>> = BTreeMap::new();

    for (path, maybe_contents) in flattened_templates {
        // Normalize the file path by replacing special characters:
        let path = PathBuf::from(
            path.to_str()
                .context("Failed to convert PathBuf to str")?
                .replace('¡', "/")
                .replace('\'', "\""),
        );
        let path_str = path.to_str().context("Failed to convert PathBuf to str")?;
        if let Some(contents) = maybe_contents {
            if EACH_TEMPLATE_REGEX.is_match(path_str) {
                handle_each_regex_template(
                    h,
                    &path,
                    path_str,
                    &contents,
                    existing_app_file_tree,
                    data,
                    &mut transformed_templates,
                )?;
            } else if IF_TEMPLATE_REGEX.is_match(path_str) {
                handle_if_template_regex(
                    h,
                    path_str,
                    &contents,
                    existing_app_file_tree,
                    data,
                    &mut transformed_templates,
                )?;
            } else {
                path.extension().map_or(Ok::<_, ScaffoldError>(()), |e| {
                    if e == "hbs" {
                        let new_path = h.render_template(
                            path.as_os_str()
                                .to_str()
                                .context("Failed to convert OsStr to str")?,
                            data,
                        )?;
                        let target_path = PathBuf::from(new_path).with_extension("");

                        let new_contents = render_template_file(
                            h,
                            existing_app_file_tree,
                            &target_path,
                            &contents,
                            &serde_json::json!(data),
                        )?;
                        let new_contents = format_code(&new_contents, &target_path)?;

                        transformed_templates.insert(target_path, Some(new_contents));
                    }
                    Ok(())
                })?;
            }
        } else {
            let new_path = h.render_template(
                path.as_os_str()
                    .to_str()
                    .context("Failed to convert OsStr to str")?,
                data,
            )?;
            transformed_templates.insert(PathBuf::from(new_path), None);
        }
    }

    unflatten_file_tree(&transformed_templates)
}

fn handle_each_regex_template<T: Serialize>(
    h: &Handlebars,
    path: &Path,
    path_str: &str,
    contents: &str,
    existing_app_file_tree: &FileTree,
    data: &T,
    transformed_templates: &mut BTreeMap<PathBuf, Option<String>>,
) -> ScaffoldResult<()> {
    let path_prefix = EACH_TEMPLATE_REGEX.replace(path_str, "${c}");
    let path_prefix = h.render_template(path_prefix.to_string().as_str(), data)?;

    let new_path_suffix = EACH_TEMPLATE_REGEX.replace(path_str, "{{#each ${b} }}${a}.hbs{{/each}}");

    let all_paths = h.render_template(new_path_suffix.as_ref(), data)?;

    let files_to_create = all_paths
        .split(".hbs")
        .filter_map(|s| {
            if s.is_empty() {
                return None;
            }
            Some(s.to_string())
        })
        .collect::<Vec<String>>();

    if files_to_create.is_empty() {
        return Ok(());
    }

    let delimiter = "\n----END_OF_FILE_DELIMITER----\n";
    let b = EACH_TEMPLATE_REGEX.replace(path_str, "${b}");
    let new_all_contents = if EACH_IF_TEMPLATE_REGEX.is_match(path_str) {
        let d = EACH_IF_TEMPLATE_REGEX.replace(path_str, "${d}");
        format!("{{{{#each {b} }}}}{{{{#if {d} }}}}\n{contents}{delimiter}{{{{/if}}}}{{{{/each}}}}",)
    } else {
        format!("{{{{#each {b} }}}}\n{contents}{delimiter}{{{{/each}}}}",)
    };
    let new_contents = render_template_file(
        h,
        existing_app_file_tree,
        path,
        &new_all_contents,
        &serde_json::json!(data),
    )?;
    let new_contents_split: Vec<String> = new_contents
        .split(delimiter)
        .map(|s| s.to_string())
        .collect();

    for (i, f) in files_to_create.into_iter().enumerate() {
        let target_path = PathBuf::from(path_prefix.clone()).join(f);
        let new_contents = format_code(&new_contents_split[i].clone(), &target_path)?;
        transformed_templates.insert(target_path, Some(new_contents));
    }
    Ok(())
}

fn handle_if_template_regex<T: Serialize>(
    h: &Handlebars,
    path_str: &str,
    contents: &str,
    existing_app_file_tree: &FileTree,
    data: &T,
    transformed_templates: &mut BTreeMap<PathBuf, Option<String>>,
) -> ScaffoldResult<()> {
    let path_prefix = IF_TEMPLATE_REGEX.replace(path_str, "${c}");
    let path_prefix = h.render_template(path_prefix.to_string().as_str(), data)?;

    let new_path_suffix = IF_TEMPLATE_REGEX.replace(path_str, "{{#if ${b} }}${a}.hbs{{/if}}");

    let new_template = h.render_template(new_path_suffix.to_string().as_str(), data)?;

    if let Some(file_name) = new_template.strip_suffix(".hbs") {
        let target_path = PathBuf::from(path_prefix.clone()).join(file_name);

        let new_contents = render_template_file(
            h,
            existing_app_file_tree,
            &target_path,
            contents,
            &serde_json::json!(data),
        )?;
        let new_contents = format_code(&new_contents, file_name)?;

        transformed_templates.insert(target_path, Some(new_contents));
    }
    Ok(())
}

pub fn render_template_file_tree_and_merge_with_existing<T: Serialize>(
    app_file_tree: FileTree,
    h: &Handlebars,
    template_file_tree: &FileTree,
    data: &T,
) -> ScaffoldResult<FileTree> {
    let rendered_templates =
        render_template_file_tree(&app_file_tree, h, template_file_tree, data)?;

    let mut flattened_app_file_tree = flatten_file_tree(&app_file_tree);
    let flattened_templates = flatten_file_tree(&rendered_templates);

    flattened_app_file_tree.extend(flattened_templates);

    unflatten_file_tree(&flattened_app_file_tree)
}
