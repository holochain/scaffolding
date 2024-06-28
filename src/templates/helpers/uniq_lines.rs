use std::collections::HashSet;

use handlebars::{
    Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext, RenderError,
    Renderable, StringOutput,
};

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
            .map(|s| s.to_string())
            .collect::<HashSet<String>>()
            .into_iter()
            .collect();

        out.write(unique_lines.join("\n").as_str())?;
        Ok(())
    }
}

pub fn register_uniq_lines(mut h: Handlebars) -> Handlebars {
    h.register_helper("uniq_lines", Box::new(UniqLines));

    h
}
