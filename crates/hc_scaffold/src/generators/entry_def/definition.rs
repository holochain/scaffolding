use convert_case::{Case, Casing};

pub fn initial_entry_def_file(entry_def: &String) -> String {
    format!(
        r#"use hdi::prelude::*;

#[hdk_entry_type]
pub struct {} {{
}}
"#,
        entry_def.to_case(Case::Title)
    )
}
