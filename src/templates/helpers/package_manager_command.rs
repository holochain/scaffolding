use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderErrorReason,
};

use crate::scaffold::web_app::npm::{Npm, SubCommand};

#[derive(Clone, Copy)]
pub struct PackageManagerCommandHelper;

impl HelperDef for PackageManagerCommandHelper {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'rc>,
        _r: &'reg Handlebars<'reg>,
        _ctx: &'rc Context,
        _rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let mut params = h.params().iter();

        let sub_command = params
            .next()
            .ok_or(RenderErrorReason::Other(
                "PackageManagerCommand helper: Param not found for index 0; must be subcommand"
                    .to_string(),
            ))?
            .value()
            .as_str()
            .ok_or(RenderErrorReason::Other(
                "PackageManagerCommand helper: failed to convert value to &str".to_string(),
            ))?;
        let sub_command = SubCommand::from(sub_command);

        let workspace = params
            .next()
            .ok_or(RenderErrorReason::Other(
                "PackageManagerCommand helper: Param not found for index 1; must be workspace"
                    .to_string(),
            ))?
            .value()
            .as_str();

        let command_string = Npm::generate_run_command_string(sub_command, workspace);
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
    fn test_package_manager_command_helper() {
        let hbs = setup_handlebars();
        let template = r#"{{(package_manager_command "install" null)}}"#;
        let s = hbs.render_template(template, &json!(null)).unwrap();
        assert_eq!("npm install", s);

        let template = r#"{{package_manager_command "package" "ui"}}"#;
        let s = hbs.render_template(template, &json!(null)).unwrap();
        assert_eq!("npm run package --workspace ui", s);

        let template = r#"{{package_manager_command "build:happ" null}}"#;
        let s = hbs.render_template(template, &json!(null)).unwrap();
        assert_eq!("npm run build:happ", s);
    }

    fn setup_handlebars<'a>() -> Handlebars<'a> {
        let hbs = Handlebars::new();
        let hbs = register_package_manager_command(hbs);
        hbs
    }
}
