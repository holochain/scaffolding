use crate::error::{ScaffoldError, ScaffoldResult};

const RESERVED_WORDS: [&str; 28] = [
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
    "AnyLinkableHash",
    "Call",
];

/// Returns an error if the given string is invalid due to it being a reserved word
pub fn check_for_reserved_words(string_to_check: &str) -> ScaffoldResult<()> {
    if RESERVED_WORDS
        .iter()
        .any(|w| string_to_check.eq_ignore_ascii_case(w))
    {
        return Err(ScaffoldError::InvalidReservedWord(
            string_to_check.to_string(),
        ));
    }
    Ok(())
}
