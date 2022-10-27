use build_fs_tree::{file, serde::Serialize};
use convert_case::{Case, Casing};
use handlebars::{handlebars_helper, Context, Handlebars};
use include_dir::{include_dir, Dir};
use std::collections::BTreeMap;
use std::ffi::OsString;
use std::path::PathBuf;

use crate::definitions::FieldType;
use crate::error::{ScaffoldError, ScaffoldResult};
use crate::file_tree::{create_dir_all, FileTree};

pub fn register_concat_helper<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    handlebars_helper!(concat: |s: Vec<String>| s.join(""));
    h.register_helper("concat", Box::new(concat));

    h
}

pub fn register_partials_helpers<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    handlebars_helper!(create_imports_partial: |s: String| {
        let mut s2 = s.clone();
        s2.push_str("/create/imports");
        s2
    });
    h.register_helper("create_imports_partial", Box::new(create_imports_partial));
    handlebars_helper!(create_render_partial: |s: String| {
        let mut s2 = s.clone();
        s2.push_str("/create/render");
        s2
    });
    h.register_helper("create_render_partial", Box::new(create_render_partial));
    handlebars_helper!(detail_imports_partial: |s: String| {
        let mut s2 = s.clone();
        s2.push_str("/detail/imports");
        s2
    });
    h.register_helper("detail_imports_partial", Box::new(detail_imports_partial));
    handlebars_helper!(detail_render_partial: |s: String| {
        let mut s2 = s.clone();
        s2.push_str("/detail/render");
        s2
    });
    h.register_helper("detail_render_partial", Box::new(detail_render_partial));

    h
}

pub fn register_ts_type_helper<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    handlebars_helper!(ts_type: |json: Json| serde_json::from_str::<FieldType>(json.to_string().as_str())?.ts_type());
    h.register_helper("ts_type", Box::new(ts_type));

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

pub fn get_templates(dir: &Dir<'_>) -> Handlebars<'static> {
    let mut handlebars = Handlebars::new();

    let templates_dir_map = walk_dir(dir);

    for (path, content) in templates_dir_map {
        if let Some(e) = path.extension() {
            if e == "hbs" {
                handlebars
                    .register_template_string(
                        path.with_extension("").as_os_str().to_str().unwrap(),
                        content,
                    )
                    .unwrap();
            }
        }
    }

    handlebars
}

pub fn register_all_partials_in_dir<'a>(
    mut h: Handlebars<'a>,
    dir: &Dir<'_>,
) -> ScaffoldResult<Handlebars<'a>> {
    let partials_dir_map = walk_dir(dir);

    for (path, content) in partials_dir_map {
        if let Some(e) = path.extension() {
            if e == "hbs" {
                h.register_partial(
                    path.with_extension("").as_os_str().to_str().unwrap(),
                    content,
                )
                .unwrap();
            }
        }
    }

    Ok(h)
}

pub fn scaffold_dir<T: Serialize>(dir: &Dir<'_>, data: &T) -> ScaffoldResult<FileTree> {
    let h = get_templates(dir);

    let mut file_tree = FileTree::Directory(BTreeMap::new());

    for (name, _template) in h.get_templates() {
        let mut p = PathBuf::from(name);
        let file_name = p.file_name().unwrap().to_os_string();
        p.pop();

        let s = h.render(name, data)?;

        create_dir_all(&mut file_tree, &p)?;

        let v: Vec<OsString> = p.iter().map(|s| s.to_os_string()).collect();
        file_tree
            .path_mut(&mut v.iter())
            .ok_or(ScaffoldError::PathNotFound(p.clone()))?
            .dir_content_mut()
            .ok_or(ScaffoldError::PathNotFound(p.clone()))?
            .insert(file_name.to_os_string(), file!(s));
    }
    Ok(file_tree)
}

fn walk_dir(dir: &Dir<'_>) -> BTreeMap<PathBuf, String> {
    let mut contents: BTreeMap<PathBuf, String> = BTreeMap::new();

    for f in dir.files() {
        if let Some(s) = f.contents_utf8() {
            contents.insert(f.path().to_path_buf(), s.to_string());
        }
    }
    for d in dir.dirs() {
        contents.extend(walk_dir(d));
    }

    contents
}
