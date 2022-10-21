use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use rand::Rng;
use std::collections::BTreeMap;

use crate::error::{ScaffoldError, ScaffoldResult};

#[derive(Debug, Clone)]
pub enum Widget {
    TextField { label: String },
    TextArea { label: String },
    DateAndTime { label: String },
    Date { label: String },
    Time { label: String },
    Slider { label: String, min: i32, max: i32 },
    RadioButton { label: String, options: Vec<String> },
    Checkbox { label: String },
    Switch { label: String },
}

impl Widget {
    pub fn rust_type(&self) -> TokenStream {
        use Widget::*;
        match self {
            TextField { .. } => quote!(String),
            TextArea { .. } => quote!(String),
            DateAndTime { .. } => quote!(u32),
            Date { .. } => quote!(u32),
            Time { .. } => quote!(u32),
            Slider { min, max, .. } => quote!(u32),
            RadioButton { label, .. } => quote!(#label),
            Checkbox { .. } => quote!(bool),
            Switch { .. } => quote!(bool),
        }
    }

    // Define a non-primitive rust type for this widget
    pub fn rust_type_definition(&self) -> Option<TokenStream> {
        use Widget::*;
        match self {
            RadioButton { label, options } => {
                let options_expressions: Vec<syn::Expr> = options
                    .iter()
                    .cloned()
                    .map(|option| {
                        let e: syn::Expr = syn::parse_str(option.to_case(Case::Title).as_str())
                            .expect("Unable to parse");
                        e
                    })
                    .collect();

                let enum_definition = quote! {enum #label {
                  #(#options_expressions),*
                }};
                Some(enum_definition)
            }
            _ => None,
        }
    }

    pub fn render_html(&self) -> String {
        format!("")
    }

    pub fn js_sample_value(&self) -> String {
        match self {
            Widget::TextArea { .. } => String::from("'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.'"),
            Widget::TextField { .. } => String::from("'Lorem ipsum'"),
            Widget::Checkbox { .. } | Widget::Switch { .. }=> String::from("true"),
            Widget::Date { .. } | Widget::DateAndTime { .. }|Widget::Time { .. }=> String::from("1665499212508"),
            Widget::Slider { min, max, .. } => {
                let mut rng = rand::thread_rng();
                format!("{}", rng.gen_range(min.clone()..max.clone()))
            },
            Widget::RadioButton {options,..} => options[0].to_case(Case::Title),
        }
    }
}

#[derive(Debug, Clone)]
pub enum HdkType {
    AgentPubKey,
    EntryHash,
    ActionHash,
}

impl HiddenType {
    pub fn rust_type(&self) -> TokenStream {
        match self {
            HiddenType::HdkType(HdkType::ActionHash) => quote!(ActionHash),
            HiddenType::HdkType(HdkType::EntryHash) => quote!(EntryHash),
            HiddenType::HdkType(HdkType::AgentPubKey) => quote!(AgentPubKey),
        }
    }
}

#[derive(Debug, Clone)]
pub enum HiddenType {
    HdkType(HdkType),
}

#[derive(Debug, Clone)]
pub enum FieldType {
    // This field will be visible in the UI when rendering this entry type
    Visible(Widget),
    // This field won't be visible in the UI when rendering this entry type
    Hidden(HiddenType),
}

impl FieldType {
    pub fn rust_type(&self) -> TokenStream {
        match self {
            FieldType::Visible(v) => v.rust_type(),
            FieldType::Hidden(hidden) => hidden.rust_type(),
        }
    }

    pub fn js_sample_value(&self) -> String {
        match self {
            FieldType::Visible(v) => v.js_sample_value(),
            // TODO: finish this
            _ => String::from(""),
        }
    }

    pub fn list_names() -> Vec<String> {
        vec![
            "TextArea",
            "TextField",
            "DateAndTime",
            "Date",
            "Time",
            "Slider",
            "RadioButton",
            "Checkbox",
            "Switch",
            "AgentPubKey",
            "EntryHash",
            "ActionHash",
        ]
        .into_iter()
        .map(|s| s.to_string())
        .collect()
    }

    pub fn from_name(name: &String) -> ScaffoldResult<FieldType> {
        match name.as_str() {
            "TextField" => Ok(FieldType::Visible(Widget::TextField {
                label: String::from(""),
            })),
            "TextArea" => Ok(FieldType::Visible(Widget::TextArea {
                label: String::from(""),
            })),
            "DateAndTime" => Ok(FieldType::Visible(Widget::DateAndTime {
                label: String::from(""),
            })),
            "Date" => Ok(FieldType::Visible(Widget::Date {
                label: String::from(""),
            })),
            "Time" => Ok(FieldType::Visible(Widget::Time {
                label: String::from(""),
            })),
            "Slider" => Ok(FieldType::Visible(Widget::Slider {
                label: String::from(""),
                min: 0,
                max: 10,
            })),
            "RadioButton" => Ok(FieldType::Visible(Widget::RadioButton {
                label: String::from(""),
                options: vec![],
            })),
            "Checkbox" => Ok(FieldType::Visible(Widget::Checkbox {
                label: String::from(""),
            })),
            "Switch" => Ok(FieldType::Visible(Widget::Switch {
                label: String::from(""),
            })),
            "AgentPubKey" => Ok(FieldType::Hidden(HiddenType::HdkType(HdkType::AgentPubKey))),
            "EntryHash" => Ok(FieldType::Hidden(HiddenType::HdkType(HdkType::EntryHash))),
            "ActionHash" => Ok(FieldType::Hidden(HiddenType::HdkType(HdkType::ActionHash))),
            _ => Err(ScaffoldError::InvalidFieldType(
                name.clone(),
                FieldType::list_names().join(", "),
            )),
        }
    }

    /// This function offers a dialoguer to the user to further configure the field type
    pub fn choose_from_name(name: &String) -> ScaffoldResult<FieldType> {
        // TODO: actually implement this
        FieldType::from_name(name)
    }
}

pub struct EntryDefinition {
    pub name: String,
    pub fields: BTreeMap<String, FieldType>,
}

impl EntryDefinition {
    pub fn render_definition_file(&self) -> TokenStream {
        let type_definitions: Vec<TokenStream> = self
            .fields
            .values()
            .filter_map(|field_type| match field_type {
                FieldType::Visible(widget) => widget.rust_type_definition(),
                _ => None,
            })
            .collect();

        let name: syn::Expr =
            syn::parse_str(self.name.to_case(Case::Title).as_str()).expect("Unable to parse");

        let fields: Vec<TokenStream> = self
            .fields
            .iter()
            .map(|(key, value)| {
                let name: syn::Expr =
                    syn::parse_str(key.to_case(Case::Snake).as_str()).expect("Unable to parse");
                let rust_type = value.rust_type();
                quote! {  #name: #rust_type }
            })
            .collect();

        quote! {
          use hdi::prelude::*;

          #(#type_definitions)*

          struct #name {
            #(#fields)*
          }
        }
    }

    pub fn js_sample_object(&self) -> String {
        let fields_samples: Vec<String> = self
            .fields
            .iter()
            .map(|(field_name, field_type)| {
                format!("{}: {}", field_name, field_type.js_sample_value())
            })
            .collect();
        format!(
            r#"{{
  {}
}}"#,
            fields_samples.join(",\n  ")
        )
    }
}

pub struct CoordinatorZomeDefinition {}

pub struct IntegrityZomeDefinition {
    pub entry_types: BTreeMap<String, EntryDefinition>,
}

pub struct DnaDefinition {}
