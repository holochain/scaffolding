use build_fs_tree::serde::Serialize;
use convert_case::{Case, Casing};
use dialoguer::theme::ColorfulTheme;
use dialoguer::Select;
use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext,
    RenderError, Renderable, ScopedJson, StringOutput,
};
use regex::Regex;
use serde_json::{json, Map, Value};
use std::collections::{BTreeMap, HashSet};
use std::ffi::OsString;
use std::path::PathBuf;

use crate::error::{ScaffoldError, ScaffoldResult};
use crate::file_tree::{
    dir_content, dir_exists, file_content, find_files, flatten_file_tree, unflatten_file_tree,
    FileTree,
};
use crate::scaffold::web_app::uis::{guess_or_choose_framework, template_for_ui_framework};

pub mod get;

pub mod collection;
pub mod coordinator;
pub mod dna;
pub mod entry_type;
pub mod example;
pub mod integrity;
pub mod link_type;
pub mod web_app;

pub struct ScaffoldedTemplate {
    pub file_tree: FileTree,
    pub next_instructions: Option<String>,
}

pub fn build_handlebars<'a>(templates_dir: &FileTree) -> ScaffoldResult<Handlebars<'a>> {
    let h = Handlebars::new();

    let mut h = register_helpers(h);

    let field_types_path = PathBuf::from("field-types");
    let v: Vec<OsString> = field_types_path.iter().map(|s| s.to_os_string()).collect();

    if let Some(field_types_templates) = templates_dir.path(&mut v.iter()) {
        h = register_all_partials_in_dir(h, field_types_templates)?;
    }
    h.register_escape_fn(handlebars::no_escape);

    Ok(h)
}

pub fn register_helpers<'a>(h: Handlebars<'a>) -> Handlebars<'a> {
    let h = register_concat_helper(h);
    let h = register_contains_helper(h);
    let h = register_includes_helper(h);
    let h = register_case_helpers(h);
    let h = register_replace_helper(h);
    let h = register_pluralize_helpers(h);
    let h = register_merge_scope(h);
    let h = register_uniq_lines(h);
    let h = register_filter(h);

    h
}

pub fn register_concat_helper<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    h.register_helper(
        "concat",
        Box::new(
            |h: &Helper,
             _r: &Handlebars,
             _: &Context,
             _rc: &mut RenderContext,
             out: &mut dyn Output|
             -> HelperResult {
                let result = h
                    .params()
                    .into_iter()
                    .map(|p| p.render())
                    .collect::<Vec<String>>()
                    .join("");

                out.write(result.as_ref())?;
                Ok(())
            },
        ),
    );

    h
}

#[derive(Clone, Copy)]
pub struct MergeScope;

fn get_scope_open_and_close_char_indexes(
    text: &String,
    scope_opener: &String,
) -> Result<(usize, usize), RenderError> {
    let mut index = text.find(scope_opener.as_str()).ok_or(RenderError::new(
        "Given scope opener not found in the given parameter",
    ))?;

    index = index + scope_opener.len() - 1;
    let scope_opener_index = index.clone();
    let mut scope_count = 1;

    while scope_count > 0 {
        index += 1;
        match text.chars().nth(index) {
            Some('{') => {
                scope_count += 1;
            }
            Some('}') => {
                scope_count -= 1;
            }
            None => {
                return Err(RenderError::new("Malformed scopes"));
            }
            _ => {}
        }
    }

    Ok((scope_opener_index, index))
}

impl HelperDef for MergeScope {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let t = h.template().ok_or(RenderError::new(
            "merge_scope helper cannot have empty content",
        ))?;

        let s = h
            .param(0)
            .ok_or(RenderError::new("merge_scope helper needs two parameters"))?
            .value()
            .as_str()
            .ok_or(RenderError::new(
                "merge_scope first parameter must be a string",
            ))?
            .to_string();
        let scope_opener = h
            .param(1)
            .ok_or(RenderError::new("merge_scope helper needs two parameters"))?
            .value()
            .as_str()
            .ok_or(RenderError::new(
                "merge_scope's second parameter must be a string",
            ))?
            .to_string();

        let (scope_opener_index, scope_close_index) =
            get_scope_open_and_close_char_indexes(&s, &scope_opener)?;

        out.write(&s[0..=scope_opener_index])?;
        let previous_scope_content = &s[(scope_opener_index + 1)..scope_close_index].to_string();

        let mut data = ctx
            .data()
            .as_object()
            .ok_or(RenderError::new("Context must be an object"))?
            .clone();
        data.insert(
            String::from("previous_scope_content"),
            Value::String(previous_scope_content.clone().trim().to_string()),
        );
        rc.set_context(Context::wraps(data)?);
        t.render(r, ctx, rc, out)?;

        out.write(&s[scope_close_index..])?;
        Ok(())
    }
}
pub fn register_merge_scope<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    h.register_helper("merge_scope", Box::new(MergeScope));

    h
}

#[derive(Clone, Copy)]
pub struct UniqLines;

impl HelperDef for UniqLines {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let t = h.template().ok_or(RenderError::new(
            "uniq_lines helper cannot have empty content",
        ))?;

        let mut string_output = StringOutput::new();
        t.render(r, ctx, rc, &mut string_output)?;

        let rendered_string = string_output.into_string()?;

        let unique_lines: Vec<String> = rendered_string
            .split('\n')
            .into_iter()
            .map(|s| s.to_string())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();

        out.write(unique_lines.join("\n").as_str())?;
        Ok(())
    }
}

pub fn register_uniq_lines<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    h.register_helper("uniq_lines", Box::new(UniqLines));

    h
}

#[derive(Clone, Copy)]
pub struct FilterHelper;

/// A Handlebars helper to filter an iterable JSON value.
/// It receives the value to be filtered and a string containing the condition predicate,
/// then uses Handlebars' truthy logic to filter the items in the value.
/// It also supports the `#if` helper's `includeZero` optional parameter.
impl HelperDef for FilterHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars<'reg>,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
    ) -> Result<ScopedJson<'reg, 'rc>, RenderError> {
        let mut params = h.params().iter();
        let value = params
            .next()
            .ok_or(RenderError::new(
                "Filter helper: Param not found for index 0; must be value to be filtered",
            ))?
            .value();

        let condition = params
            .next()
            .ok_or(RenderError::new("Filter helper: Param not found for index 1; must be string containing filter condition predicate"))?
            .value()
            .as_str()
            .ok_or(RenderError::new("Filter helper: filter condition predicate must be a string"))?;

        let include_zero = h
            .hash_get("includeZero")
            .and_then(|v| v.value().as_bool())
            .unwrap_or(false);

        // This template allows us to evaluate the condition according to
        // Handlebars' available context/property logic, helper functions, and
        // truthiness logic.
        let template = format!(
            "{}{}{}{}",
            "{{#if ",
            condition,
            include_zero.then_some(" includeZero=true").unwrap_or(""),
            "}}true{{else}}false{{/if}}"
        );

        match value {
            Value::Array(items) => {
                let mut filtered_array = vec![];
                for item in items.iter() {
                    match r.render_template(&template, &item) {
                        Ok(s) => {
                            if s.as_str() == "true" {
                                filtered_array.push(item);
                            }
                        }
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                Ok(ScopedJson::Derived(json!(filtered_array)))
            }
            Value::Object(object) => {
                let mut filtered_object = Map::new();
                for key in object.clone().keys() {
                    if let Some(v) = object.get(key) {
                        match r.render_template(&template, &v) {
                            Ok(s) => {
                                if s.as_str() == "true" {
                                    filtered_object.insert(key.into(), v.clone());
                                }
                            }
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
                Ok(ScopedJson::Derived(json!(filtered_object)))
            }
            _ => Err(RenderError::new(
                "Filter helper: value to be filtered must be an array or object",
            )),
        }
    }
}

pub fn register_filter<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    h.register_helper("filter", Box::new(FilterHelper));

    h
}

pub fn register_contains_helper<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    handlebars_helper!(contains: |list: Option<Vec<Value>>, value: Value| list.is_some() && list.unwrap().contains(&value));
    h.register_helper("contains", Box::new(contains));

    h
}

pub fn register_includes_helper<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    handlebars_helper!(includes: |string: String, substring: String| string.contains(&substring));
    h.register_helper("includes", Box::new(includes));

    h
}

pub fn register_replace_helper<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    handlebars_helper!(replace: |s: String, pattern: String, replaced_by: String| s.replace(&pattern, replaced_by.as_str()));
    h.register_helper("replace", Box::new(replace));

    h
}

pub fn register_pluralize_helpers<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    handlebars_helper!(singular: |s: String| pluralizer::pluralize(s.as_str(), 1, false));
    h.register_helper("singular", Box::new(singular));
    handlebars_helper!(plural: |s: String| pluralizer::pluralize(s.as_str(), 2, false));
    h.register_helper("plural", Box::new(plural));

    h
}

pub fn register_case_helpers<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    handlebars_helper!(title_case: |s: String| s.to_case(Case::Title));
    h.register_helper("title_case", Box::new(title_case));

    handlebars_helper!(lower_case: |s: String| s.to_case(Case::Lower));
    h.register_helper("lower_case", Box::new(lower_case));

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
    let partials = find_files(file_tree, &|path, _contents| {
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
            content.trim(),
        )?;
    }

    Ok(h)
}

pub fn render_template_file<'a>(
    h: &Handlebars<'a>,
    existing_app_file_tree: &FileTree,
    target_path: &PathBuf,
    template_str: &String,
    value: &serde_json::Value,
) -> ScaffoldResult<String> {
    let mut value = value.clone();

    if let Ok(previous_content) = file_content(existing_app_file_tree, &target_path) {
        value
            .as_object_mut()
            .unwrap()
            .insert("previous_file_content".into(), previous_content.into());
    }

    let mut h = h.clone();
    h.register_template_string(target_path.to_str().unwrap(), template_str)?;
    let new_contents = h.render(target_path.to_str().unwrap(), &value)?;

    Ok(new_contents)
}

pub fn render_template_file_tree<'a, T: Serialize>(
    existing_app_file_tree: &FileTree,
    h: &Handlebars<'a>,
    templates_file_tree: &FileTree,
    data: &T,
) -> ScaffoldResult<FileTree> {
    let flattened_templates = flatten_file_tree(templates_file_tree);

    let mut transformed_templates: BTreeMap<PathBuf, Option<String>> = BTreeMap::new();

    let new_data = serde_json::to_string(data)?;
    let value: serde_json::Value = serde_json::from_str(new_data.as_str())?;

    for (path, maybe_contents) in flattened_templates {
        let path = PathBuf::from(path.to_str().unwrap().replace('¡', "/"));
        let path = PathBuf::from(path.to_str().unwrap().replace('\'', "\""));
        if let Some(contents) = maybe_contents {
            let re = Regex::new(
                r"(?P<c>(.)*)/\{\{#each (?P<b>([^\{\}])*)\}\}(?P<a>(.)*)\{\{/each\}\}.hbs\z",
            )
            .unwrap();
            let if_regex = Regex::new(
                r"(?P<c>(.)*)/\{\{#if (?P<b>([^\{\}])*)\}\}(?P<a>(.)*)\{\{/if\}\}.hbs\z",
            )
            .unwrap();

            if re.is_match(path.to_str().unwrap()) {
                let path_prefix = re.replace(path.to_str().unwrap(), "${c}");
                let path_prefix = h.render_template(path_prefix.to_string().as_str(), data)?;

                let new_path_suffix =
                    re.replace(path.to_str().unwrap(), "{{#each ${b} }}${a}.hbs{{/each}}");

                let all_paths = h.render_template(new_path_suffix.to_string().as_str(), data)?;

                let files_to_create: Vec<String> = all_paths
                    .split(".hbs")
                    .map(|s| s.to_string())
                    .filter(|s| !s.is_empty())
                    .collect();

                if files_to_create.len() > 0 {
                    let delimiter = "\n----END_OF_FILE_DELIMITER----\n";

                    let each_if_re = Regex::new(
                    r"(?P<c>(.)*)/\{\{#each (?P<b>([^\{\}])*)\}\}\{\{#if (?P<d>([^\{\}])*)\}\}(?P<a>(.)*)\{\{/if\}\}\{\{/each\}\}.hbs\z",
                )
                .unwrap();
                    let b = re.replace(path.to_str().unwrap(), "${b}");
                    let new_all_contents = match each_if_re.is_match(path.to_str().unwrap()) {
                        true => {
                            let d = each_if_re.replace(path.to_str().unwrap(), "${d}");
                            format!(
                                "{{{{#each {} }}}}{{{{#if {} }}}}\n{}{}{{{{/if}}}}{{{{/each}}}}",
                                b, d, contents, delimiter
                            )
                        }

                        false => format!(
                            "{{{{#each {} }}}}\n{}{}{{{{/each}}}}",
                            b, contents, delimiter
                        ),
                    };
                    let new_contents = render_template_file(
                        &h,
                        existing_app_file_tree,
                        &path,
                        &new_all_contents,
                        &value,
                    )?;
                    let new_contents_split: Vec<String> = new_contents
                        .split(delimiter)
                        .into_iter()
                        .map(|s| s.to_string())
                        .collect();

                    for (i, f) in files_to_create.into_iter().enumerate() {
                        let target_path = PathBuf::from(path_prefix.clone()).join(f);

                        transformed_templates
                            .insert(target_path, Some(new_contents_split[i].clone()));
                    }
                }
            } else if if_regex.is_match(path.to_str().unwrap()) {
                let path_prefix = if_regex.replace(path.to_str().unwrap(), "${c}");
                let path_prefix = h.render_template(path_prefix.to_string().as_str(), data)?;

                let new_path_suffix =
                    if_regex.replace(path.to_str().unwrap(), "{{#if ${b} }}${a}.hbs{{/if}}");

                let new_template = h.render_template(new_path_suffix.to_string().as_str(), data)?;

                if let Some(file_name) = new_template.strip_suffix(".hbs") {
                    let target_path = PathBuf::from(path_prefix.clone()).join(file_name);

                    let new_contents = render_template_file(
                        &h,
                        existing_app_file_tree,
                        &target_path,
                        &contents,
                        &value,
                    )?;
                    transformed_templates.insert(target_path, Some(new_contents));
                }
            } else if let Some(e) = path.extension() {
                if e == "hbs" {
                    let new_path = h.render_template(path.as_os_str().to_str().unwrap(), data)?;
                    let target_path = PathBuf::from(new_path).with_extension("");

                    let new_contents = render_template_file(
                        &h,
                        existing_app_file_tree,
                        &target_path,
                        &contents,
                        &value,
                    )?;

                    transformed_templates.insert(target_path, Some(new_contents));
                }
            }
        } else {
            let new_path = h.render_template(path.as_os_str().to_str().unwrap(), data)?;
            transformed_templates.insert(PathBuf::from(new_path), None);
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
    let rendered_templates =
        render_template_file_tree(&app_file_tree, h, template_file_tree, data)?;

    let mut flattened_app_file_tree = flatten_file_tree(&app_file_tree);
    let flattened_templates = flatten_file_tree(&rendered_templates);

    flattened_app_file_tree.extend(flattened_templates);

    unflatten_file_tree(&flattened_app_file_tree)
}

pub fn templates_path() -> PathBuf {
    PathBuf::from(".templates")
}

pub fn choose_or_get_template_file_tree(
    file_tree: &FileTree,
    template: &Option<String>,
) -> ScaffoldResult<FileTree> {
    if dir_exists(file_tree, &templates_path()) {
        let template_name = choose_or_get_template(file_tree, template)?;

        Ok(FileTree::Directory(dir_content(
            &file_tree,
            &templates_path().join(template_name),
        )?))
    } else {
        let ui_framework = guess_or_choose_framework(file_tree)?;

        template_for_ui_framework(&ui_framework)
    }
}

pub fn choose_or_get_template(
    file_tree: &FileTree,
    template: &Option<String>,
) -> ScaffoldResult<String> {
    let templates_path = PathBuf::new().join(templates_path());

    let templates_dir_content =
        dir_content(file_tree, &templates_path).map_err(|_e| ScaffoldError::NoTemplatesFound)?;

    let templates = templates_dir_content
        .iter()
        .filter_map(|(k, v)| {
            if v.file_content().is_some() {
                return None;
            }
            k.to_str().map(|s| s.to_string())
        })
        .collect::<Vec<String>>();

    let chosen_template_name = match (template, templates.len()) {
        (_, 0) => Err(ScaffoldError::NoTemplatesFound),
        (None, 1) => Ok(templates[0].clone()),
        (None, _) => {
            let option = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Which template should we use?")
                .default(0)
                .items(&templates[..])
                .interact()?;

            Ok(templates[option].clone())
        }
        (Some(t), _) => match templates.contains(&t) {
            true => Ok(t.clone()),
            false => Err(ScaffoldError::TemplateNotFound(t.clone())),
        },
    }?;

    Ok(chosen_template_name)
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    fn setup_handlebars<'a>() -> Handlebars<'a> {
        let hbs = Handlebars::new();
        let hbs = register_filter(hbs);
        hbs
    }

    #[test]
    fn test_get_scope_open_and_close_char_indexes() {
        let text = String::from("const s = {};");
        let scope_opener = String::from("const s = {");

        let (scope_opener_index, scope_close_index) =
            get_scope_open_and_close_char_indexes(&text, &scope_opener).unwrap();

        assert_eq!(scope_opener_index, 10);
        assert_eq!(scope_close_index, 11);
    }

    #[test]
    fn respects_include_zero() {
        let hbs = setup_handlebars();
        let value = json!([0, 1, 0, 2, 0, 3, 0, 4, 0, 5]);
        // The predicate filters out zeroes.
        let template = "{{#each (filter this \"this\")}}{{this}}{{/each}}";
        match hbs.render_template(&template, &value) {
            Ok(s) => assert_eq!(s, "12345", "`filter` helper did not filter out falsy zero"),
            Err(e) => panic!("{}", e)
        }
        // This predicate, however, does not.
        let template = "{{#each (filter this \"this\" includeZero=true)}}{{this}}{{/each}}";
        match hbs.render_template(&template, &value) {
            Ok(s) => assert_eq!(s, "0102030405", "`filter` helper did not treat zero as truthy"),
            Err(e) => panic!("{}", e)
        }
    }

    #[test]
    fn can_filter_object_by_value() {
        let hbs = setup_handlebars();
        let value = json!({"name": "Alice", "age": 24, "wild": false, "species": "iguana"});
        // The predicate filters out the 'wild' property.
        let template = "{{#each (filter this \"this\")}}{{@key}}: {{this}}, {{/each}}";
        match hbs.render_template(&template, &value) {
            Ok(s) => assert_eq!(s, "name: Alice, age: 24, species: iguana, ", "`filter` helper did not filter object key/value pairs by value"),
            Err(e) => panic!("{}", e)
        }
    }


    #[test]
    fn can_filter_complex_value() {
        let hbs = setup_handlebars();
        let value = json!([
            {"name": "Alice", "age": 24, "wild": true, "species": "iguana"},
            {"name": "Bob", "age": 3, "wild": false, "species": "hamster"},
            {"name": "Carol", "age": 1, "wild": true, "species": "octopus"}
        ]);
        // The predicate filters out domestic animals.
        let template = "{{#each (filter this \"wild\")}}{{name}} the {{species}} is {{age}}. {{/each}}";
        match hbs.render_template(&template, &value) {
            Ok(s) => assert_eq!(s, "Alice the iguana is 24. Carol the octopus is 1. ", "`filter` helper did not operate on a list full of complex values"),
            Err(e) => panic!("{}", e)
        }
    }
}
