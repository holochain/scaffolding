use convert_case::{Case, Casing};

use crate::error::{ScaffoldError, ScaffoldResult};

const RESERVED_WORDS: [&str; 27] = [
    "type",
    "role",
    "enum",
    "pub",
    "fn",
    "mod",
    "struct",
    "const",
    "Option",
    "Result",
    "crate",
    "hdi",
    "hdk",
    "return",
    "if",
    "else",
    "match",
    "Action",
    "Entry",
    "Record",
    "Zome",
    "Dna",
    "EntryType",
    "EntryHash",
    "ActionHash",
    "AgentPubKey",
    "Call",
];

// Returns an error if the given string is invalid due to it being a reserved word
pub fn check_for_reserved_words(string_to_check: &String) -> ScaffoldResult<()> {
    for w in RESERVED_WORDS {
        if string_to_check
            .to_case(Case::Lower)
            .eq(&w.to_string().to_case(Case::Lower))
        {
            return Err(ScaffoldError::InvalidReservedWord(w.to_string()));
        }
    }

    Ok(())
}
