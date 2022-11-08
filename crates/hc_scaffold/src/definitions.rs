use proc_macro2::TokenStream;
use quote::quote;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::scaffold::entry_type::DependsOnItself;

#[derive(Deserialize, Debug, Clone, Serialize)]
pub enum FieldType {
    #[serde(rename = "bool")]
    Bool,
    String,
    #[serde(rename = "u32")]
    U32,
    #[serde(rename = "i32")]
    I32,
    #[serde(rename = "f32")]
    F32,
    Timestamp,
    AgentPubKey,
    ActionHash,
    EntryHash,
}

impl ToString for FieldType {
    fn to_string(&self) -> String {
        use FieldType::*;
        match self {
            Bool => "bool",
            String => "String",
            U32 => "u32",
            I32 => "i32",
            F32 => "f32",
            Timestamp => "Timestamp",
            ActionHash => "ActionHash",
            EntryHash => "EntryHash",
            AgentPubKey => "AgentPubKey",
        }
        .into()
    }
}

impl FieldType {
    pub fn list() -> Vec<FieldType> {
        vec![
            FieldType::String,
            FieldType::Bool,
            FieldType::U32,
            FieldType::I32,
            FieldType::F32,
            FieldType::Timestamp,
            FieldType::ActionHash,
            FieldType::EntryHash,
            FieldType::AgentPubKey,
        ]
    }

    pub fn rust_type(&self) -> TokenStream {
        use FieldType::*;
        match self {
            Bool => quote!(bool),
            String => quote!(String),
            U32 => quote!(u32),
            I32 => quote!(i32),
            F32 => quote!(f32),
            Timestamp => quote!(Timestamp),
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
        match self {
           FieldType::String => String::from("'Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed nec eros quis enim hendrerit aliquet.'"),
            FieldType::Bool => String::from("true"),
            FieldType::Timestamp => String::from("1665499212508"),
            FieldType::U32 => {
                let mut rng = rand::thread_rng();
                format!("{}", rng.gen_range(0..100))
            },
            FieldType::I32 => {
                let mut rng = rand::thread_rng();
                format!("{}", rng.gen_range(-100..100))
            },
            FieldType::F32 => {
                let mut rng = rand::thread_rng();
                format!("{}", rng.gen_range(-100.0..100.0))
            },
            FieldType::ActionHash => format!(
                    "Buffer.from(new Uint8Array([{}]))",
                    vec![0x84, 0x29, 0x24]
                        .into_iter()
                        .chain(vec![0x00; 36].into_iter())
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            FieldType::EntryHash => format!(
                    "Buffer.from(new Uint8Array([{}]))",
                    vec![0x84, 0x21, 0x24]
                        .into_iter()
                        .chain(vec![0x00; 36].into_iter())
                        .map(|i| i.to_string())
                        .collect::<Vec<String>>()
                        .join(", ")
                ),
            FieldType::AgentPubKey => format!(
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
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Cardinality {
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "vector")]
    Vector,
    #[serde(rename = "option")]
    Option,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FieldDefinition {
    pub field_name: String,
    pub field_type: FieldType,
    pub widget: Option<String>,
    pub cardinality: Cardinality,
}

impl FieldDefinition {
    pub fn rust_type(&self) -> TokenStream {
        match self.cardinality {
            Cardinality::Single => self.field_type.rust_type(),
            Cardinality::Option => {
                let rust_representation_type = self.field_type.rust_type();

                quote! {Option<#rust_representation_type>}
            }
            Cardinality::Vector => {
                let rust_representation_type = self.field_type.rust_type();

                quote! {Vec<#rust_representation_type>}
            }
        }
    }

    pub fn js_sample_value(&self) -> String {
        match self.cardinality {
            Cardinality::Vector => format!(
                "[{}]",
                vec![
                    self.field_type.js_sample_value(),
                    self.field_type.js_sample_value(),
                    self.field_type.js_sample_value()
                ]
                .join(", ")
            ),
            Cardinality::Single | Cardinality::Option => self.field_type.js_sample_value(),
        }
    }
}

#[derive(Serialize, Clone)]
pub struct EntryDefinition {
    pub singular_name: String,
    pub plural_name: String,
    pub fields: Vec<FieldDefinition>,
    pub depends_on: Vec<String>,
    pub depends_on_itself: DependsOnItself,
}

impl EntryDefinition {
    pub fn js_sample_object(&self) -> String {
        let fields_samples: Vec<String> = self
            .fields
            .iter()
            .map(|field_def| {
                format!(
                    "{}: {}",
                    field_def.field_name,
                    field_def.field_type.js_sample_value()
                )
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
