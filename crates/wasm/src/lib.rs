mod utils;

use serde::{Deserialize, Serialize};
use tsify::Tsify;
use wasm_bindgen::prelude::*;
use holochain_scaffolding_utils::FileTree as NativeFileTree;
use holochain_scaffolding_utils::{file, dir};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen(typescript_custom_section)]
const TS_APPEND_CONTENT: &'static str = r#"
export type NativeFileTree = {[x: string]: NativeFileTree | string}; 
"#;

#[derive(Tsify, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[tsify(into_wasm_abi, from_wasm_abi)]
pub struct FileTree(pub NativeFileTree);

#[wasm_bindgen]
pub fn greet() -> FileTree {
    FileTree(dir! {
        "index.html" => file!(r#"
            <!DOCTYPE html>
            <link rel="stylesheet" href="styles/style.css" />
            <script src="scripts/main.js"></script>
        "#)
        "scripts" => dir! {
            "main.js" => file!(r#"document.write('Hello World')"#)
        }
        "styles" => dir! {
            "style.css" => file!(r#":root { color: red; }"#)
        }
    })
}