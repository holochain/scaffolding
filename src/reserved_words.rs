use std::collections::HashSet;

use crate::error::{ScaffoldError, ScaffoldResult};

/// Returns an error if the given string is invalid due to it being a reserved word
pub fn check_for_reserved_keywords(string_to_check: &str) -> ScaffoldResult<()> {
    let mut reserved_holochain_words_set: HashSet<&str> =
        HashSet::with_capacity(HOLOCHAIN_RESERVED_KEYWORDS.len());
    reserved_holochain_words_set.extend(HOLOCHAIN_RESERVED_KEYWORDS);

    let mut reserved_rust_words_set: HashSet<&str> =
        HashSet::with_capacity(RUST_RESERVED_KEYWORDS.len());
    reserved_rust_words_set.extend(RUST_RESERVED_KEYWORDS);

    if reserved_holochain_words_set.contains(string_to_check.to_ascii_lowercase().as_str()) {
        return Err(ScaffoldError::InvalidReservedWord {
            context: "holochain".to_string(),
            word: string_to_check.to_string(),
        });
    }

    if reserved_rust_words_set.contains(string_to_check.to_ascii_lowercase().as_str()) {
        return Err(ScaffoldError::InvalidReservedWord {
            context: "rust".to_string(),
            word: string_to_check.to_string(),
        });
    }

    Ok(())
}

const HOLOCHAIN_RESERVED_KEYWORDS: [&str; 16] = [
    "role",
    "hdi",
    "hdk",
    "action",
    "entry",
    "record",
    "zome",
    "dna",
    "entrytype",
    "entryhash",
    "actionhash",
    "agentpubkey",
    "anylinkablehash",
    "holohash",
    "externalhash",
    "call",
];

// <https://doc.rust-lang.org/reference/keywords.html>
const RUST_RESERVED_KEYWORDS: [&str; 50] = [
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for",
    "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return",
    "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where",
    "while", "abstract", "async", "await", "become", "box", "do", "final", "macro", "override",
    "priv", "try", "typeof", "unsized", "virtual", "yield",
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_for_reserved_keywords_works() {
        let valid = check_for_reserved_keywords("Value");
        assert!(valid.is_ok());

        let invalid = check_for_reserved_keywords("static");
        assert!(invalid.is_err());

        let invalid = check_for_reserved_keywords("EntryType");
        assert!(invalid.is_err());
    }
}
