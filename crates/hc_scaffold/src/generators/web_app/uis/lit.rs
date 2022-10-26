use handlebars::Handlebars;
use include_dir::{include_dir, Dir};

use crate::{
    definitions::EntryDefinition,
    error::ScaffoldResult,
    file_tree::FileTree,
    templates::{register_case_helpers, register_ts_type_helper, scaffold_dir},
};

use super::{AddEntryTypeComponentsData, ScaffoldWebAppData};

static LIT_WEB_APP: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/uis/lit/web-app");
static CREATE_ENTRY_COMPONENT: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/templates/uis/lit/create-entry-type.hbs"
));

pub fn scaffold_lit_web_app(data: &ScaffoldWebAppData) -> ScaffoldResult<FileTree> {
    scaffold_dir(&LIT_WEB_APP, data)
}

pub fn create_entry_component(data: &AddEntryTypeComponentsData) -> ScaffoldResult<String> {
    let h = Handlebars::new();

    let h = register_case_helpers(h);
    let h = register_ts_type_helper(h);

    let s = h.render_template(CREATE_ENTRY_COMPONENT, data)?;

    Ok(s)
}
