use build_fs_tree::serde::Serialize;
use dialoguer::{theme::ColorfulTheme, Select};
use handlebars::Handlebars;
use include_dir::{include_dir, Dir};
use std::{ffi::OsString, path::PathBuf, str::FromStr};

use crate::{
    definitions::EntryDefinition,
    error::{ScaffoldError, ScaffoldResult},
    file_tree::{dir_to_file_tree, FileTree},
    templates::{
        register_all_partials_in_dir, register_helpers,
        render_template_file_tree_and_merge_with_existing,
    },
    versions::holochain_client_version,
};

static LIT_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/lit");
static SVELTE_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/svelte");
static VUE_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/vue");
static VANILLA_TEMPLATES: Dir<'static> = include_dir!("$CARGO_MANIFEST_DIR/templates/vanilla");

#[derive(Debug, Clone)]
pub enum UiFramework {
    Vanilla,
    Lit,
    Svelte,
    Vue,
}

impl FromStr for UiFramework {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> ScaffoldResult<UiFramework> {
        match s {
            "vanilla" => Ok(UiFramework::Vanilla),
            "svelte" => Ok(UiFramework::Svelte),
            "vue" => Ok(UiFramework::Vue),
            "lit" => Ok(UiFramework::Lit),
            _ => Err(ScaffoldError::InvalidUiFramework(
                s.to_string(),
                "vanilla, lit, svelte, vue".to_string(),
            )),
        }
    }
}

pub fn choose_ui_framework() -> ScaffoldResult<UiFramework> {
    let frameworks = vec!["Vanilla", "Lit", "Svelte", "Vue"];
    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Choose UI framework:")
        .default(0)
        .items(&frameworks[..])
        .interact()?;

    UiFramework::from_str(frameworks[selection].to_lowercase().as_str())
}

pub fn template_for_ui_framework(framework: &UiFramework) -> ScaffoldResult<FileTree> {
    let dir = match framework {
        UiFramework::Lit => &LIT_TEMPLATES,
        UiFramework::Vanilla => &VANILLA_TEMPLATES,
        UiFramework::Svelte => &SVELTE_TEMPLATES,
        UiFramework::Vue => &VUE_TEMPLATES,
    };

    dir_to_file_tree(dir)
}
