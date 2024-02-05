use std::path::{Component, Path, PathBuf};

use handlebars::{Handlebars, HelperDef, RenderError, ScopedJson};

use crate::file_tree::{file_exists, load_directory_into_memory};

#[derive(Clone, Copy)]
/// A handlebars helper to check whether a given file exists in the current app file tree
pub struct FileExistsHelper;

impl HelperDef for FileExistsHelper {
    fn call_inner<'reg: 'rc, 'rc>(
        &self,
        h: &handlebars::Helper<'reg, 'rc>,
        r: &'reg handlebars::Handlebars<'reg>,
        ctx: &'rc handlebars::Context,
        _: &mut handlebars::RenderContext<'reg, 'rc>,
    ) -> Result<handlebars::ScopedJson<'reg, 'rc>, handlebars::RenderError> {
        let search_path_str = h
            .params()
            .iter()
            .next()
            .ok_or(RenderError::new("Missing search path param"))?
            .value()
            .to_string();
        let data = ctx.data();
        let search_path_str = r.render_template(&search_path_str, data)?.replace("\"", "");

        let current_ui_dir = std::env::current_dir()
            .map_err(|_| RenderError::new("current working dir is invalid"))?
            .join("ui");

        let ui_file_tree = load_directory_into_memory(&current_ui_dir)
            .map_err(|_| RenderError::new("Faild to load directory into memory"))?;

        let needle = PathBuf::from(search_path_str);
        let needle = normalize_path(needle.as_path());
        let exists = file_exists(&ui_file_tree, &needle);
        Ok(ScopedJson::Derived(serde_json::Value::Bool(exists)))
    }
}

pub fn register_file_exists<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    h.register_helper("file_exists", Box::new(FileExistsHelper));

    h
}

pub fn normalize_path(path: &Path) -> PathBuf {
    let mut components = path.components().peekable();
    let mut ret = if let Some(c @ Component::Prefix(..)) = components.peek().cloned() {
        components.next();
        PathBuf::from(c.as_os_str())
    } else {
        PathBuf::new()
    };

    for component in components {
        match component {
            Component::Prefix(..) => unreachable!(),
            Component::RootDir => {
                ret.push(component.as_os_str());
            }
            Component::CurDir => {}
            Component::ParentDir => {
                ret.pop();
            }
            Component::Normal(c) => {
                ret.push(c);
            }
        }
    }
    ret
}
