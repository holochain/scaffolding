use proc_macro2::TokenStream;
use quote::quote;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(tag = "type")]
pub enum FieldType {
    TextField,
    TextArea,
    DateAndTime,
    Date,
    Slider { min: i32, max: i32 },
    Checkbox,
    Switch,
    AgentPubKey,
    ActionHash,
    EntryHash,
}

impl ToString for FieldType {
    fn to_string(&self) -> String {
        use FieldType::*;
        match self {
            TextField => "TextField",
            TextArea => "TextArea",
            DateAndTime => "DateAndTime",
            Date => "Date",
            Slider { .. } => "Slider",
            Checkbox => "Checkbox",
            Switch => "Switch",
            ActionHash => "ActionHash",
            EntryHash => "EntryHash",
            AgentPubKey => "AgentPubKey",
        }
        .into()
    }
}

impl FieldType {
    pub fn rust_type(&self) -> TokenStream {
        use FieldType::*;
        match self {
            TextField => quote!(String),
            TextArea => quote!(String),
            DateAndTime => quote!(u32),
            Date => quote!(u32),
            Slider { .. } => quote!(u32),
            Checkbox => quote!(bool),
            Switch => quote!(bool),
            ActionHash => quote!(ActionHash),
            EntryHash => quote!(EntryHash),
            AgentPubKey => quote!(AgentPubKey),
        }
    }

    // Define a non-primitive rust type for this widget
    pub fn rust_type_definition(&self) -> Option<TokenStream> {
        match self {
            // RadioButton { label, options } => {
            //     let options_expressions: Vec<syn::Expr> = options
            //         .iter()
            //         .cloned()
            //         .map(|option| {
            //             let e: syn::Expr = syn::parse_str(option.to_case(Case::Pascal).as_str())
            //                 .expect("Unable to parse");
            //             e
            //         })
            //         .collect();

            //     let enum_definition = quote! {enum #label {
            //       #(#options_expressions),*
            //     }};
            //     Some(enum_definition)
            // }
            _ => None,
        }
    }

    pub fn js_sample_value(&self) -> String {
        use FieldType::*;
        match self {
            TextArea => String::from("'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.'"),
            TextField => String::from("'Lorem ipsum'"),
            Checkbox  | Switch { .. } => String::from("true"),
            Date  | DateAndTime => String::from("1665499212508"),
            Slider { min, max } => {
                let mut rng = rand::thread_rng();
                format!("{}", rng.gen_range(min.clone()..max.clone()))
            },
            ActionHash => format!(
                    "Buffer.from(new Uint8Array([{}]))",
                    vec![0x84, 0x29, 0x24]
                        .into_iter()
                        .chain(vec![0x00; 36].into_iter())
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            EntryHash => format!(
                    "Buffer.from(new Uint8Array([{}]))",
                    vec![0x84, 0x21, 0x24]
                        .into_iter()
                        .chain(vec![0x00; 36].into_iter())
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            AgentPubKey => format!(
                    "Buffer.from(new Uint8Array([{}]))",
                    vec![0x84, 0x20, 0x24]
                        .into_iter()
                        .chain(vec![0x00; 36].into_iter())
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
            )
        }
    }

    pub fn ts_type(&self) -> String {
        use FieldType::*;
        match self {
            TextArea => String::from("string"),
            TextField => String::from("string"),
            Checkbox | Switch => String::from("boolean"),
            Slider { .. } | Date | DateAndTime => String::from("number"),
            // get::RadioButton { options, .. } => options
            //     .iter()
            //     .map(|s| format!("'{}'", s))
            //     .collect::<Vec<String>>()
            //     .join(" | "),
            ActionHash => String::from("ActionHash"),
            EntryHash => String::from("EntryHash"),
            AgentPubKey => String::from("AgentPubKey"),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldDefinition {
    pub field_type: FieldType,
    pub label: String,
    pub vector: bool,
}

impl FieldDefinition {
    pub fn rust_type(&self) -> TokenStream {
        match self.vector {
            false => self.field_type.rust_type(),
            true => {
                let rust_representation_type = self.field_type.rust_type();

                quote! {Vec<#rust_representation_type>}
            }
        }
    }

    pub fn js_sample_value(&self) -> String {
        if self.vector {
            format!(
                "[{}]",
                vec![
                    self.field_type.js_sample_value(),
                    self.field_type.js_sample_value(),
                    self.field_type.js_sample_value()
                ]
                .join(", ")
            )
        } else {
            self.field_type.js_sample_value()
        }
    }
}

#[derive(Serialize, Clone)]
pub struct EntryDefinition {
    pub name: String,
    pub fields: BTreeMap<String, FieldDefinition>,
}

impl EntryDefinition {
    pub fn js_sample_object(&self) -> String {
        let fields_samples: Vec<String> = self
            .fields
            .iter()
            .map(|(field_name, field_def)| {
                format!("{}: {}", field_name, field_def.field_type.js_sample_value())
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
