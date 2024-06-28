use std::str::FromStr;

use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
};

use crate::scaffold::web_app::package_manager::{PackageManager, SubCommand};

#[derive(Clone, Copy)]
pub struct PackageManagerCommandHelper;

impl HelperDef for PackageManagerCommandHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        _r: &'reg Handlebars<'reg>,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let mut params = h.params().iter();
        let package_manager = params
            .next()
            .ok_or(RenderError::new(
                "PackageManagerCommand helper: Param not found for index 0; must be the package manager",
            ))?
            .value()
            .as_str()
            .ok_or(RenderError::new(
                "PackageManagerCommand helper: failed to convert value to &str",
            ))?;
        let package_manager = PackageManager::from_str(package_manager)
            .map_err(|e| RenderError::new(format!("Invalid package manager: {e}")))?;

        let sub_command = params
            .next()
            .ok_or(RenderError::new(
                "PackageManagerCommand helper: Param not found for index 1; must be subcommand",
            ))?
            .value()
            .as_str()
            .ok_or(RenderError::new(
                "PackageManagerCommand helper: failed to convert value to &str",
            ))?;
        let sub_command = SubCommand::from(sub_command);

        let workspace = params
            .next()
            .ok_or(RenderError::new(
                "PackageManagerCommand helper: Param not found for index 3; must be workspace",
            ))?
            .value()
            .as_str();

        let command_string = package_manager.run_command_string(sub_command, workspace);
        out.write(&command_string)?;
        Ok(())
    }
}

pub fn register_package_manager_command(mut h: Handlebars) -> Handlebars {
    h.register_helper(
        "package_manager_command",
        Box::new(PackageManagerCommandHelper),
    );

    h
}

#[cfg(test)]
mod tests {
    use super::*;
    use handlebars::Handlebars;
    use serde_json::json;

    #[test]
    fn test_package_manager_command_helper_yarn() {
        let hbs = setup_handlebars();
        let template = r#"{{(package_manager_command "yarn" "install" null)}}"#;
        let s = hbs.render_template(template, &json!(null)).unwrap();
        assert_eq!("yarn install", s);

        let template = r#"{{package_manager_command "yarn" "package" "ui"}}"#;
        let s = hbs.render_template(template, &json!(null)).unwrap();
        assert_eq!("yarn workspace ui package", s);

        let template = r#"{{package_manager_command "yarn" "build:happ" null}}"#;
        let s = hbs.render_template(template, &json!(null)).unwrap();
        assert_eq!("yarn build:happ", s);
    }

    #[test]
    fn test_package_manager_command_helper_with_invalid_package_manager() {
        let hbs = setup_handlebars();
        // invalid/ unsupported package manager
        let template = r#"{{(package_manager_command "unknown" "install" null)}}"#;
        if let Err(e) = hbs.render_template(template, &json!(null)) {
            assert!(e.to_string().contains("Invalid package manager"));
        };
    }

    fn setup_handlebars<'a>() -> Handlebars<'a> {
        let hbs = Handlebars::new();
        let hbs = register_package_manager_command(hbs);
        hbs
    }
}
