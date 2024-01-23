use handlebars::{
    Context, Handlebars, Helper, HelperDef, RenderContext, RenderError,
    ScopedJson,
};
use serde_json::{json, Value, Map};

#[derive(Clone, Copy)]
pub struct FilterHelper;

pub enum FilterableValues {
    Array(Vec<Value>),
    Object(Map<String, Value>),
}

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

        let items: FilterableValues = match value {
            Value::Array(items) => Ok(
                FilterableValues::Array(items
                    .iter()
                    .cloned()
                    .collect()
                )
            ),
            Value::Object(items) => Ok(
                FilterableValues::Object(items.clone())
            ),
            _ => Err(RenderError::new("Filter helper: value to be filtered must be an array or object"))
        }?;

        // This template allows us to evaluate the condition according to Handlebars'
        // available helper functions and existing truthiness logic.
        let template = format!(
            "{}{}{}{}",
            "{{#if ",
            include_zero.then_some("includeZero=true").unwrap_or(""),
            condition,
            "}}true{{else}}false{{/if}}"
        );

        match items {
            FilterableValues::Array(items) => {
                let mut filtered_array = vec![];
                for item in items.iter() {
                    match r.render_template(&template, item) {
                        Ok(s) => {
                            if s.as_str() == "true" {
                                filtered_array.push(item);
                            }
                        },
                        Err(e) => {
                            return Err(e);
                        }
                    }
                }
                Ok(ScopedJson::Derived(json!(filtered_array)))
            },
            FilterableValues::Object(object) => {
                let mut filtered_object = Map::new();
                for key in object.keys() {
                    if let Some(v) = object.get(key) {
                        match r.render_template(&template, v) {
                            Ok(s) => {
                                if s.as_str() == "true" {
                                    filtered_object.insert(key.into(), v.clone());
                                }
                            },
                            Err(e) => {
                                return Err(e);
                            }
                        }
                    }
                }
                Ok(ScopedJson::Derived(json!(filtered_object)))
            }
        }
    }
}

pub fn register_filter<'a>(mut h: Handlebars<'a>) -> Handlebars<'a> {
    h.register_helper("filter", Box::new(FilterHelper));

    h
}
