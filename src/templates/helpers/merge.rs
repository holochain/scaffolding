use std::collections::{BTreeMap, HashSet};
use std::ffi::OsString;
use std::path::PathBuf;

use handlebars::{
    handlebars_helper, Context, Handlebars, Helper, HelperDef, HelperResult, Output, RenderContext,
    RenderError, Renderable, StringOutput,
};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Number, Value};

pub fn register_merge<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    h.register_helper("merge", Box::new(Merge));
    h.register_helper("match_scope", Box::new(MatchScope));

    h
}

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

    let mut whitespace = true;

    while whitespace {
        match text.chars().nth(index - 1) {
            Some(' ') => {
                index -= 1;
            }
            _ => {
                whitespace = false;
            }
        }
    }

    Ok((scope_opener_index, index))
}

#[derive(Clone, Copy)]
pub struct Merge;

impl HelperDef for Merge {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        out: &mut dyn Output,
    ) -> HelperResult {
        let t = h
            .template()
            .ok_or(RenderError::new("merge helper cannot have empty content"))?;

        let s = h
            .param(0)
            .ok_or(RenderError::new("merge helper needs 1 parameter"))?
            .value()
            .as_str()
            .ok_or(RenderError::new("merge first parameter must be a string"))?
            .to_string();

        let mut data = ctx
            .data()
            .as_object()
            .ok_or(RenderError::new("Context must be an object"))?
            .clone();
        data.insert(String::from(SCOPE_CONTENT), Value::String(s.clone()));
        rc.set_context(Context::wraps(data)?);

        let mut inner_output = StringOutput::new();
        t.render(r, ctx, rc, &mut inner_output)?;

        if let Some(context) = rc.context() {
            let mut data = context
                .data()
                .as_object()
                .ok_or(RenderError::new("Context must be an object"))?
                .clone();
            if let Some(Value::Array(matched_scopes)) = data.get(MATCHED_SCOPES) {
                let mut previous_index = s.len();

                let mut matched_scopes: Vec<MatchedScopedData> = matched_scopes
                    .into_iter()
                    .filter_map(|ms| serde_json::from_value::<MatchedScopedData>(ms.clone()).ok())
                    .collect();

                matched_scopes.sort_by(|a, b| b.__starting_index.cmp(&a.__starting_index));

                let mut full_merge_content = String::from("");
                for matched_scope in matched_scopes {
                    let mut full_scope_content = String::from("");
                    let start_index = matched_scope.__starting_index;
                    full_scope_content.push_str(matched_scope.__new_scope_content.as_str());
                    full_scope_content.push_str(
                        &s[(start_index + matched_scope.__old_scope_length)..previous_index],
                    );
                    previous_index = start_index + 1;
                    full_merge_content.insert_str(0, full_scope_content.as_str());
                }
                full_merge_content.insert_str(0, &s[0..=previous_index]);
                out.write(&full_merge_content)?;

                data.remove(MATCHED_SCOPES);
                rc.set_context(Context::wraps(data)?);
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug)]
struct MatchedScopedData {
    __starting_index: usize,
    __new_scope_content: String,
    __old_scope_length: usize,
}

const MATCHED_SCOPES: &'static str = "__matched_scopes";
const SCOPE_CONTENT: &'static str = "__scope_content";
const STARTING_INDEX: &'static str = "__starting_index";
const NEW_SCOPE_CONTENT: &'static str = "__new_scope_content";
const OLD_SCOPE_LENGTH: &'static str = "__old_scope_length";

#[derive(Clone, Copy)]
pub struct MatchScope;

impl HelperDef for MatchScope {
    fn call<'reg: 'rc, 'rc>(
        &self,
        h: &Helper<'reg, 'rc>,
        r: &'reg Handlebars<'reg>,
        ctx: &'rc Context,
        rc: &mut RenderContext<'reg, 'rc>,
        _out: &mut dyn Output,
    ) -> HelperResult {
        let t = h
            .template()
            .ok_or(RenderError::new("merge helper cannot have empty content"))?;

        let mut data = rc
            .context()
            .unwrap()
            .data()
            .as_object()
            .ok_or(RenderError::new("Context must be an object"))?
            .clone();

        let Some(Value::String(scope_content)) = data.get(SCOPE_CONTENT) else {
            return Err(RenderError::new(
            "match_scope needs to be placed inside a merge helper",
            ));
        };

        let scope_opener = h
            .param(0)
            .ok_or(RenderError::new("merge helper needs 1 parameter"))?
            .value()
            .as_str()
            .ok_or(RenderError::new("merge's first parameter must be a string"))?
            .to_string();

        let (scope_opener_index, scope_close_index) =
            get_scope_open_and_close_char_indexes(&scope_content, &scope_opener)?;

        let previous_scope_content =
            &scope_content[(scope_opener_index + 1)..scope_close_index].to_string();

        data.insert(
            String::from("previous_scope_content"),
            Value::String(previous_scope_content.clone().trim().to_string()),
        );
        data.insert(
            String::from("untrimmed_previous_scope_content"),
            Value::String(previous_scope_content.clone().to_string()),
        );

        let mut matched_scopes = match data.get(MATCHED_SCOPES) {
            Some(Value::Array(array)) => array.clone(),
            _ => vec![],
        };

        rc.set_context(Context::wraps(data.clone())?);

        let mut inner_output = StringOutput::new();
        t.render(r, ctx, rc, &mut inner_output)?;

        let out_string = inner_output.into_string().unwrap();

        let mut map = Map::new();
        map.insert(
            String::from(STARTING_INDEX),
            Value::Number(Number::from(scope_opener_index)),
        );

        map.insert(String::from(NEW_SCOPE_CONTENT), Value::String(out_string));
        map.insert(
            String::from(OLD_SCOPE_LENGTH),
            Value::Number(Number::from(scope_close_index - scope_opener_index)),
        );

        matched_scopes.push(Value::Object(map));

        data.insert(MATCHED_SCOPES.to_string(), Value::Array(matched_scopes));

        rc.set_context(Context::wraps(data)?);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;
    use handlebars::Handlebars;
    use serde_json::{Map, Value};

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
    fn test_merge_match_scope_simple() {
        let h = Handlebars::new();

        let h = register_merge(h);

        let code = String::from(
            r#"class A {
    // Multiline
    // Comment
}
"#,
        );
        let mut map = Map::new();
        map.insert(
            String::from("previous_file_content"),
            Value::String(String::from(code)),
        );
        let context = Context::from(Value::Object(map));
        let template = r#"
{{#merge previous_file_content}}
    {{#match_scope "class A {"}}
    nestedFn() {

    }
    {{previous_scope_content}}
    {{/match_scope}}
{{/merge}}
"#;

        assert_eq!(
            h.render_template_with_context(template, &context).unwrap(),
            r#"
class A {
    nestedFn() {

    }
    // Multiline
    // Comment
}
"#,
        );
    }

    #[test]
    fn test_merge_match_scope() {
        let h = Handlebars::new();

        let h = register_merge(h);

        let code = String::from(
            r#"export class A {
    nestedFn1() {

    }
}
export class B {
    nestedFn() {
        // First line
    }
}
"#,
        );
        let mut map = Map::new();
        map.insert(
            String::from("previous_file_content"),
            Value::String(String::from(code)),
        );
        map.insert(
            String::from("class_functions"),
            Value::Array(vec![
                Value::String(String::from("nestedFn2")),
                Value::String(String::from("nestedFn3")),
            ]),
        );
        let context = Context::from(Value::Object(map));
        let template = r#"{{#merge previous_file_content}}
    {{#match_scope "export class B {"}}
        {{#merge untrimmed_previous_scope_content}}
            {{#match_scope "nestedFn() {"}}
        {{previous_scope_content}}
        // New line
            {{/match_scope}}
        {{/merge}}
    {{/match_scope}}
    {{#match_scope "export class A {"}}
        {{#each class_functions}}
    {{this}}() {

    }
        {{/each}}
    {{previous_scope_content}}
    {{/match_scope}}
{{/merge}}
"#;

        assert_eq!(
            h.render_template_with_context(template, &context).unwrap(),
            r#"export class A {
    nestedFn2() {

    }
    nestedFn3() {

    }
    nestedFn1() {

    }
}
export class B {
    nestedFn() {
        // First line
        // New line
    }
}
"#,
        );
    }
}
