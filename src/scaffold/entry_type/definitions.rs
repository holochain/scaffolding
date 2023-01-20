use convert_case::{Case, Casing};
use proc_macro2::TokenStream;
use quote::quote;
use rand::Rng;
use serde::{ser::SerializeStruct, Deserialize, Serialize, Serializer};

use crate::{
    error::{ScaffoldError, ScaffoldResult},
    utils::check_snake_case,
};

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
        )))
    }
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

#[derive(Serialize, Debug, Clone)]
pub struct FieldDefinition {
    pub field_name: String,
    pub field_type: FieldType,
    pub widget: Option<String>,
    pub cardinality: Cardinality,
    pub linked_from: Option<Referenceable>,
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

pub fn parse_entry_type_reference(s: &str) -> ScaffoldResult<EntryTypeReference> {
    let sp: Vec<&str> = s.split(":").collect();
    check_snake_case(&sp[0].to_string(), "entry type reference")?;

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

pub fn parse_referenceable(s: &str) -> ScaffoldResult<Referenceable> {
    let sp: Vec<&str> = s.split(":").collect();

    check_snake_case(&sp[0].to_string(), "referenceable")?;

    Ok(match sp[0] {
        "agent" => match sp.len() {
            0 | 1 => Referenceable::Agent {
                role: String::from("agent"),
            },
            _ => Referenceable::Agent {
                role: sp[1].to_string(),
            },
        },
        _ => Referenceable::EntryType(parse_entry_type_reference(s)?),
    })
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
            _ => pluralizer::pluralize(singular.as_str(), 1, false),
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
}
