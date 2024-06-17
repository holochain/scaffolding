use std::str::FromStr;

use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::{error::ScaffoldError, reserved_words::check_for_reserved_words, utils::check_case};

#[derive(Deserialize, Debug, Clone, Serialize)]
#[serde(tag = "type")]
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
    DnaHash,
    Enum {
        label: String,
        variants: Vec<String>,
    },
}

impl TryFrom<String> for FieldType {
    type Error = ScaffoldError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let list = FieldType::list();

        for el in list {
            if value.eq(&el.to_string()) {
                return Ok(el);
            }
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
            FieldType::U32 => "u32",
            FieldType::I32 => "i32",
            FieldType::F32 => "f32",
            FieldType::Timestamp => "Timestamp",
            FieldType::ActionHash => "ActionHash",
            FieldType::EntryHash => "EntryHash",
            FieldType::DnaHash => "DnaHash",
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
            FieldType::U32,
            FieldType::I32,
            FieldType::F32,
            FieldType::Timestamp,
            FieldType::ActionHash,
            FieldType::EntryHash,
            FieldType::DnaHash,
            FieldType::AgentPubKey,
            FieldType::Enum {
                label: String::from(""),
                variants: vec![],
            },
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
            DnaHash => quote!(DnaHash),
            EntryHash => quote!(EntryHash),
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
            U32 => "number",
            I32 => "number",
            F32 => "number",
            Timestamp => "number",
            AgentPubKey => "AgentPubKey",
            ActionHash => "ActionHash",
            EntryHash => "EntryHash",
            DnaHash => "DnaHash",
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
        check_for_reserved_words(&field_name)?;
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

#[derive(Serialize, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EntryTypeReference {
    pub entry_type: String,
    pub reference_entry_hash: bool,
}

impl EntryTypeReference {
    pub fn hash_type(&self) -> FieldType {
        match self.reference_entry_hash {
            true => FieldType::EntryHash,
            false => FieldType::ActionHash,
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

    pub fn to_string(&self, c: &Cardinality) -> String {
        match c {
            Cardinality::Vector => pluralizer::pluralize(self.entry_type.as_str(), 2, false),
            _ => pluralizer::pluralize(self.entry_type.as_str(), 1, false),
        }
    }
}

impl FromStr for EntryTypeReference {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sp: Vec<&str> = s.split(':').collect();
        check_case(sp[0], "entry type reference", Case::Snake)?;

        let reference_entry_hash = match sp.len() {
            0 | 1 => false,
            _ => match sp[1] {
                "EntryHash" => true,
                "ActionHash" => false,
                _ => Err(ScaffoldError::InvalidArguments(String::from(
                    "second argument for reference type must be \"EntryHash\" or \"ActionHash\"",
                )))?,
            },
        };

        Ok(EntryTypeReference {
            entry_type: sp[0].to_string().to_case(Case::Pascal),
            reference_entry_hash,
        })
    }
}

#[derive(Clone, Debug)]
pub enum Referenceable {
    Agent { role: String },
    EntryType(EntryTypeReference),
}

impl Serialize for Referenceable {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("Referenceable", 3)?;
        state.serialize_field("name", &self.to_string(&Cardinality::Single))?;
        state.serialize_field("hash_type", &self.hash_type().to_string())?;
        state.serialize_field("singular_arg", &self.field_name(&Cardinality::Single))?;
        state.end()
    }
}

impl FromStr for Referenceable {
    type Err = ScaffoldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sp: Vec<&str> = s.split(':').collect();

        check_case(sp[0], "referenceable", Case::Snake)?;

        Ok(match sp[0] {
            "agent" => match sp.len() {
                0 | 1 => Referenceable::Agent {
                    role: String::from("agent"),
                },
                _ => Referenceable::Agent {
                    role: sp[1].to_string(),
                },
            },
            _ => Referenceable::EntryType(EntryTypeReference::from_str(s)?),
        })
    }
}

impl Referenceable {
    pub fn hash_type(&self) -> FieldType {
        match self {
            Referenceable::Agent { .. } => FieldType::AgentPubKey,
            Referenceable::EntryType(r) => r.hash_type(),
        }
    }

    pub fn field_name(&self, c: &Cardinality) -> String {
        let s = self.to_string(c).to_case(Case::Snake);

        match self {
            Referenceable::Agent { .. } => s,
            Referenceable::EntryType(e) => e.field_name(c),
        }
    }

    pub fn to_string(&self, c: &Cardinality) -> String {
        let singular = match self {
            Referenceable::Agent { role } => role.clone(),
            Referenceable::EntryType(r) => r.entry_type.clone(),
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
                    format!(
                        "  {}: Array<{}>;",
                        &field.field_name.to_case(Case::Snake),
                        ts_type
                    )
                }
            };
            ts_interface.push_str(&ts_field);
            ts_interface.push('\n');
        }
        ts_interface.push('}');
        ts_enums
            .is_empty()
            .then_some(ts_interface.clone())
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
