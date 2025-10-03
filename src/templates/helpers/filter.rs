use handlebars::{Context, Handlebars, Helper, HelperDef, RenderContext, RenderError, ScopedJson};
use serde_json::{json, Map, Value};

/// A Handlebars helper to filter an iterable JSON value.
/// It receives the value to be filtered and a string containing the condition predicate,
/// then uses Handlebars' truthy logic to filter the items in the value.
/// It also supports the `#if` helper's `includeZero` optional parameter.
#[derive(Clone, Copy)]
pub struct FilterHelper;

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
            if include_zero {
                " includeZero=true"
            } else {
                Default::default()
            },
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

pub fn register_filter(mut h: Handlebars) -> Handlebars {
    h.register_helper("filter", Box::new(FilterHelper));

    h
}

#[cfg(test)]
mod tests {
    use crate::templates::helpers::register_filter;
    use handlebars::Handlebars;
    use serde_json::json;

    fn setup_handlebars<'a>() -> Handlebars<'a> {
        let hbs = Handlebars::new();
        let hbs = register_filter(hbs);
        hbs
    }

    #[test]
    fn respects_include_zero() {
        let hbs = setup_handlebars();
        let value = json!([0, 1, 0, 2, 0, 3, 0, 4, 0, 5]);
        // The predicate filters out zeroes.
        let template = "{{#each (filter this \"this\")}}{{this}}{{/each}}";
        match hbs.render_template(template, &value) {
            Ok(s) => assert_eq!(s, "12345", "`filter` helper did not filter out falsy zero"),
            Err(e) => panic!("{}", e),
        }
        // This predicate, however, does not.
        let template = "{{#each (filter this \"this\" includeZero=true)}}{{this}}{{/each}}";
        match hbs.render_template(template, &value) {
            Ok(s) => assert_eq!(
                s, "0102030405",
                "`filter` helper did not treat zero as truthy"
            ),
            Err(e) => panic!("{}", e),
        }
    }

    #[test]
    fn can_filter_object_by_value() {
        let hbs = setup_handlebars();
        let value = json!({"name": "Alice", "age": 24, "wild": false, "species": "iguana"});
        // The predicate filters out the 'wild' property.
        let template = "{{#each (filter this \"this\")}}{{@key}}: {{this}}, {{/each}}";
        match hbs.render_template(template, &value) {
            Ok(s) => assert_eq!(
                s, "name: Alice, age: 24, species: iguana, ",
                "`filter` helper did not filter object key/value pairs by value"
            ),
            Err(e) => panic!("{}", e),
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
        let template =
            "{{#each (filter this \"wild\")}}{{name}} the {{species}} is {{age}}. {{/each}}";
        match hbs.render_template(template, &value) {
            Ok(s) => assert_eq!(
                s, "Alice the iguana is 24. Carol the octopus is 1. ",
                "`filter` helper did not operate on a list full of complex values"
            ),
            Err(e) => panic!("{}", e),
        }
    }
}
