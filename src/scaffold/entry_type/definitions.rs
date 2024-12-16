use anyhow::Context;
use colored::Colorize;
use convert_case::{Case, Casing};
use itertools::Itertools;
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use regex::Regex;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};
use std::str::FromStr;

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    reserved_words::check_for_reserved_keywords,
    utils::check_case,
};

#[derive(Deserialize, Debug, Clone, Serialize, Eq, PartialEq)]
#[serde(tag = "type")]
pub enum FieldType {
    #[serde(rename = "bool")]
    Bool,
    String,
    #[serde(rename = "u8")]
    U8,
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
    DnaHash,
    ExternalHash,
    Enum {
        label: String,
        variants: Vec<String>,
    },
}

impl FromStr for FieldType {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(f) = FieldType::list().iter().find(|v| s == v.to_string()) {
            return Ok(f.to_owned());
        }

        Err(ScaffoldError::InvalidArguments(format!(
            "Invalid field type: only {:?} are allowed",
            FieldType::list()
                .into_iter()
                .map(|ft| ft.to_string())
                .collect::<String>()
        )))
    }
}

impl std::fmt::Display for FieldType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            FieldType::Bool => "bool",
            FieldType::String => "String",
            FieldType::U8 => "u8",
            FieldType::U32 => "u32",
            FieldType::I32 => "i32",
            FieldType::F32 => "f32",
            FieldType::Timestamp => "Timestamp",
            FieldType::ActionHash => "ActionHash",
            FieldType::EntryHash => "EntryHash",
            FieldType::DnaHash => "DnaHash",
            FieldType::ExternalHash => "ExternalHash",
            FieldType::AgentPubKey => "AgentPubKey",
            FieldType::Enum { .. } => "Enum",
        };
        write!(f, "{str}")
    }
}

impl FieldType {
    pub fn list() -> Vec<FieldType> {
        vec![
            FieldType::String,
            FieldType::Bool,
            FieldType::U8,
            FieldType::U32,
            FieldType::I32,
            FieldType::F32,
            FieldType::Timestamp,
            FieldType::ActionHash,
            FieldType::EntryHash,
            FieldType::DnaHash,
            FieldType::ExternalHash,
            FieldType::AgentPubKey,
            FieldType::Enum {
                label: String::new(),
                variants: Vec::new(),
            },
        ]
    }

    pub fn parse_enum(fields_str: &str) -> ScaffoldResult<FieldType> {
        let mut str_path = fields_str.split(':');

        let variants = str_path
            .next_back()
            .context(format!("Enum variants missing from: {}", fields_str))?;
        let variants = variants
            .split('.')
            .map(|v| v.to_case(Case::Pascal))
            .collect::<Vec<_>>();
        let label = str_path
            .next_back()
            .context(format!("Enum label missing from: {}", fields_str))?
            .to_string();

        Ok(FieldType::Enum { label, variants })
    }

    pub fn rust_type(&self) -> TokenStream {
        use FieldType::*;

        match self {
            Bool => quote!(bool),
            String => quote!(String),
            U8 => quote!(u8),
            U32 => quote!(u32),
            I32 => quote!(i32),
            F32 => quote!(f32),
            Timestamp => quote!(Timestamp),
            ActionHash => quote!(ActionHash),
            DnaHash => quote!(DnaHash),
            EntryHash => quote!(EntryHash),
            ExternalHash => quote!(ExternalHash),
            AgentPubKey => quote!(AgentPubKey),
            Enum { label, .. } => {
                let ident = format_ident!("{}", label);
                quote!(#ident)
            }
        }
    }

    pub fn ts_type(&self) -> &str {
        use FieldType::*;

        match self {
            Bool => "boolean",
            String => "string",
            U8 => "number",
            U32 => "number",
            I32 => "number",
            F32 => "number",
            Timestamp => "number",
            AgentPubKey => "AgentPubKey",
            ActionHash => "ActionHash",
            EntryHash => "EntryHash",
            DnaHash => "DnaHash",
            ExternalHash => "ExternalHash",
            Enum { label, .. } => label,
        }
    }

    // Define a non-primitive rust type for this widget
    pub fn rust_type_definition(&self) -> Option<TokenStream> {
        match self {
            FieldType::Enum { label, variants } => {
                let variants_expressions: Vec<syn::Expr> = variants
                    .iter()
                    .cloned()
                    .map(|variant| {
                        let e: syn::Expr = syn::parse_str(variant.to_case(Case::Pascal).as_str())
                            .expect("Unable to parse");
                        e
                    })
                    .collect();

                let label_ident = format_ident!("{}", label);
                let enum_definition = quote! {
                    #[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
                    #[serde(tag = "type")]
                    pub enum #label_ident {
                      #(#variants_expressions),*
                    }
                };
                Some(enum_definition)
            }
            _ => None,
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

#[derive(Serialize, Debug, Clone)]
pub struct FieldDefinition {
    pub field_name: String,
    pub field_type: FieldType,
    pub widget: Option<String>,
    pub cardinality: Cardinality,
    pub linked_from: Option<Referenceable>,
}

impl FieldDefinition {
    pub fn new(
        field_name: String,
        field_type: FieldType,
        widget: Option<String>,
        cardinality: Cardinality,
        linked_from: Option<Referenceable>,
    ) -> Result<Self, ScaffoldError> {
        check_for_reserved_keywords(&field_name)?;
        Ok(FieldDefinition {
            field_name,
            field_type,
            widget,
            cardinality,
            linked_from,
        })
    }
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
}

impl FromStr for FieldDefinition {
    type Err = ScaffoldError;

    fn from_str(fields_str: &str) -> Result<Self, Self::Err> {
        let mut str_path = fields_str.split(':');

        let field_name = str_path.next().context(format!(
            "field_name is missing from: {}\nExample: \"{}\"",
            fields_str,
            "title:String".italic()
        ))?;
        check_case(field_name, "field_name", Case::Snake)?;

        let field_type_str = str_path.next().context(format!(
            "{} is missing a field_type, use one of: {}\nExample: \"{}\"",
            field_name,
            FieldType::list()
                .iter()
                .map(|f| f.to_string())
                .join(", ")
                .italic(),
            "title:String".italic()
        ))?;

        let vec_regex = Regex::new(r"Vec<(?P<a>(.)*)>\z").unwrap();
        let option_regex = Regex::new(r"Option<(?P<a>(.)*)>\z").unwrap();

        let (field_type, cardinality) = if vec_regex.is_match(field_type_str) {
            let field_type = vec_regex.replace(field_type_str, "${a}");

            if field_type == "Enum" {
                (FieldType::parse_enum(fields_str)?, Cardinality::Vector)
            } else {
                (FieldType::from_str(&field_type)?, Cardinality::Vector)
            }
        } else if option_regex.is_match(field_type_str) {
            let field_type = option_regex.replace(field_type_str, "${a}");

            if field_type == "Enum" {
                (FieldType::parse_enum(fields_str)?, Cardinality::Option)
            } else {
                (FieldType::from_str(&field_type)?, Cardinality::Option)
            }
        } else if field_type_str == "Enum" {
            (FieldType::parse_enum(fields_str)?, Cardinality::Single)
        } else {
            (FieldType::from_str(field_type_str)?, Cardinality::Single)
        };

        // XXX: perhaps widget-types can be validated at this level rather than
        //      on attemting to render templates
        let widget = str_path
            .next()
            .filter(|v| !v.is_empty())
            .map(|v| v.to_string());

        let linked_from = str_path
            .next()
            .filter(|v| !v.is_empty())
            .map(|v| match field_type {
                FieldType::AgentPubKey => Some(Referenceable::Agent {
                    role: v.to_string(),
                }),
                FieldType::EntryHash | FieldType::ActionHash => {
                    Some(Referenceable::EntryType(EntryTypeReference {
                        entry_type: v.to_string(),
                        reference_entry_hash: matches!(field_type, FieldType::EntryHash),
                    }))
                }
                _ => None,
            })
            .unwrap_or_default();

        FieldDefinition::new(
            field_name.to_string(),
            field_type,
            widget,
            cardinality,
            linked_from,
        )
    }
}

#[derive(Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntryTypeReference {
    pub entry_type: String,
    pub reference_entry_hash: bool,
}

impl EntryTypeReference {
    pub fn field_type(&self) -> FieldType {
        if self.reference_entry_hash {
            FieldType::EntryHash
        } else {
            FieldType::ActionHash
        }
    }

    pub fn field_name(&self, cardinality: &Cardinality) -> String {
        match cardinality {
            Cardinality::Vector => format!(
                "{}_hashes",
                pluralizer::pluralize(self.entry_type.as_str(), 2, false).to_case(Case::Snake)
            ),
            _ => format!("{}_hash", self.entry_type.to_case(Case::Snake)),
        }
    }

    pub fn name_by_cardinality(&self, c: &Cardinality) -> String {
        match c {
            Cardinality::Vector => pluralizer::pluralize(self.entry_type.as_str(), 2, false),
            _ => pluralizer::pluralize(self.entry_type.as_str(), 1, false),
        }
    }
}

impl FromStr for EntryTypeReference {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut str_path = s.split(':');

        let entry_type = str_path
            .next()
            .context(format!("Failed to parse entry_type from: {}", s))?;

        let reference_entry_hash = str_path
            .next()
            .map(|v| matches!(v, "EntryHash"))
            .unwrap_or_default();

        Ok(EntryTypeReference {
            entry_type: entry_type.to_case(Case::Pascal),
            reference_entry_hash,
        })
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum Referenceable {
    Agent { role: String },
    EntryType(EntryTypeReference),
    ExternalHash { name: String },
}

impl Serialize for Referenceable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Referenceable", 3)?;
        state.serialize_field("name", &self.to_string(&Cardinality::Single))?;
        state.serialize_field("hash_type", &self.field_type().to_string())?;
        state.serialize_field("singular_arg", &self.field_name(&Cardinality::Single))?;
        state.end()
    }
}

impl FromStr for Referenceable {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        let type_name = parts
            .first()
            .context(format!("The first argument in '{}' is invalid", s))?;

        check_case(type_name, "referenceable", Case::Snake)?;

        match *type_name {
            "agent" => {
                let role = parts.get(1).unwrap_or(&"agent").to_string();
                Ok(Referenceable::Agent { role })
            }
            _ => {
                if parts.get(1) == Some(&"ExternalHash") {
                    Ok(Referenceable::ExternalHash {
                        name: type_name.to_string(),
                    })
                } else {
                    EntryTypeReference::from_str(s).map(Referenceable::EntryType)
                }
            }
        }
    }
}

impl Referenceable {
    pub fn field_type(&self) -> FieldType {
        match self {
            Referenceable::Agent { .. } => FieldType::AgentPubKey,
            Referenceable::EntryType(r) => r.field_type(),
            Referenceable::ExternalHash { .. } => FieldType::ExternalHash,
        }
    }

    pub fn field_name(&self, c: &Cardinality) -> String {
        let s = self.to_string(c).to_case(Case::Snake);

        match self {
            Referenceable::Agent { .. } | Referenceable::ExternalHash { .. } => s,
            Referenceable::EntryType(e) => e.field_name(c),
        }
    }

    pub fn to_string(&self, c: &Cardinality) -> String {
        let singular = match self {
            Referenceable::Agent { role } => role.clone(),
            Referenceable::EntryType(r) => r.entry_type.clone(),
            Referenceable::ExternalHash { name } => name.clone(),
        };

        match c {
            Cardinality::Vector => pluralizer::pluralize(singular.as_str(), 2, false),
            _ => singular,
        }
    }
}

#[derive(Serialize, Clone, Debug)]
pub struct EntryDefinition {
    pub name: String,
    pub fields: Vec<FieldDefinition>,
    pub reference_entry_hash: bool,
}

impl EntryDefinition {
    pub fn referenceable(&self) -> Referenceable {
        Referenceable::EntryType(EntryTypeReference {
            entry_type: self.name.clone(),
            reference_entry_hash: self.reference_entry_hash,
        })
    }

    pub fn snake_case_name(&self) -> String {
        self.name.to_case(Case::Snake)
    }

    pub fn pascal_case_name(&self) -> String {
        self.name.to_case(Case::Pascal)
    }

    pub fn camel_case_name(&self) -> String {
        self.name.to_case(Case::Camel)
    }

    /// Generate entry definition as typescript interface
    pub fn ts_type_codegen(&self) -> String {
        let mut ts_interface = format!("export interface {} {{\n", &self.pascal_case_name());
        let mut ts_enums = String::new();

        for field in &self.fields {
            let ts_type = field.field_type.ts_type();
            if let FieldType::Enum { label, variants } = &field.field_type {
                let enum_definition = format!(
                    "export type {label} = {};\n",
                    variants
                        .iter()
                        .map(|v| format!("{{type: '{}'}}", v))
                        .collect::<Vec<_>>()
                        .join(" | ")
                );
                ts_enums.push_str(&enum_definition);
            }
            let ts_field = match field.cardinality {
                Cardinality::Single => {
                    format!("  {}: {};", &field.field_name.to_case(Case::Snake), ts_type)
                }
                Cardinality::Option => format!(
                    "  {}: {} | undefined;",
                    &field.field_name.to_case(Case::Snake),
                    ts_type
                ),
                Cardinality::Vector => {
                    if matches!(field.field_type, FieldType::U8) {
                        format!("  {}: Uint8Array;", &field.field_name.to_case(Case::Snake),)
                    } else {
                        format!(
                            "  {}: Array<{}>;",
                            &field.field_name.to_case(Case::Snake),
                            ts_type
                        )
                    }
                }
            };
            ts_interface.push_str(&ts_field);
            ts_interface.push('\n');
        }
        ts_interface.push('}');
        ts_enums
            .is_empty()
            .then(|| ts_interface.clone())
            .unwrap_or(format!("{ts_enums}\n{}", ts_interface.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_def_ts_codegen_with_primitive_fields() {
        let post_entry = EntryDefinition {
            name: "post".to_string(),
            fields: vec![
                FieldDefinition {
                    field_name: "title".to_string(),
                    field_type: FieldType::String,
                    widget: Some("TextField".to_string()),
                    cardinality: Cardinality::Single,
                    linked_from: None,
                },
                FieldDefinition {
                    field_name: "content".to_string(),
                    field_type: FieldType::String,
                    widget: Some("TextArea".to_string()),
                    cardinality: Cardinality::Single,
                    linked_from: None,
                },
            ],
            reference_entry_hash: false,
        };

        let comment_entry = EntryDefinition {
            name: "post".to_string(),
            fields: vec![
                FieldDefinition {
                    field_name: "comment".to_string(),
                    field_type: FieldType::String,
                    widget: Some("TextArea".to_string()),
                    cardinality: Cardinality::Single,
                    linked_from: None,
                },
                FieldDefinition {
                    field_name: "post_hash".to_string(),
                    field_type: FieldType::ActionHash,
                    widget: None,
                    cardinality: Cardinality::Single,
                    linked_from: Some(Referenceable::EntryType(EntryTypeReference {
                        entry_type: post_entry.name.to_string(),
                        reference_entry_hash: false,
                    })),
                },
            ],
            reference_entry_hash: false,
        };

        let post_ts_interface = &post_entry.ts_type_codegen();
        let expected_post_ts_interface = r#"export interface Post {
  title: string;
  content: string;
}"#;
        assert_eq!(expected_post_ts_interface, post_ts_interface);

        let comment_ts_interface = &comment_entry.ts_type_codegen();
        let expected_comment_ts_inteface = r#"export interface Post {
  comment: string;
  post_hash: ActionHash;
}"#;
        assert_eq!(expected_comment_ts_inteface, comment_ts_interface)
    }

    #[test]
    fn test_entry_def_ts_codegen_with_enums_arrays_arrays_and_options() {
        let other_entry = EntryDefinition {
            name: "example_entry".to_string(),
            fields: vec![
                FieldDefinition {
                    field_name: "field_one".to_string(),
                    field_type: FieldType::String,
                    widget: None,
                    cardinality: Cardinality::Single,
                    linked_from: None,
                },
                FieldDefinition {
                    field_name: "field_two".to_string(),
                    field_type: FieldType::U32,
                    widget: None,
                    cardinality: Cardinality::Option,
                    linked_from: None,
                },
                FieldDefinition {
                    field_name: "field_three".to_string(),
                    field_type: FieldType::Bool,
                    widget: None,
                    cardinality: Cardinality::Vector,
                    linked_from: None,
                },
                FieldDefinition {
                    field_name: "enum_field".to_string(),
                    field_type: FieldType::Enum {
                        label: "ExampleEnum".to_string(),
                        variants: vec![
                            "Variant1".to_string(),
                            "Variant2".to_string(),
                            "Variant3".to_string(),
                        ],
                    },
                    widget: None,
                    cardinality: Cardinality::Single,
                    linked_from: None,
                },
            ],
            reference_entry_hash: false,
        };

        let ts_interface = &other_entry.ts_type_codegen();

        let expected_ts_interface = r#"export type ExampleEnum = {type: 'Variant1'} | {type: 'Variant2'} | {type: 'Variant3'};

export interface ExampleEntry {
  field_one: string;
  field_two: number | undefined;
  field_three: Array<boolean>;
  enum_field: ExampleEnum;
}"#;

        assert_eq!(ts_interface, expected_ts_interface);
    }
}
