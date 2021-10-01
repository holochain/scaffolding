export default () =>
  `use hdk::prelude::*;
use hdk::prelude::holo_hash::*;

#[hdk_entry(id = "post")]
pub struct Post(String);

entry_defs![
    Post::entry_def()
];

#[hdk_extern]
pub fn create_post(post: Post) -> ExternResult<EntryHashB64> {
    create_entry(&post)?;
    let hash = hash_entry(&post)?;

    Ok(EntryHashB64::from(hash))
}

#[hdk_extern]
pub fn get_post(entry_hash: EntryHashB64) -> ExternResult<Post> {
    let element = get(EntryHash::from(entry_hash), GetOptions::default())?.ok_or(WasmError::Guest(String::from("Post not found")))?;

    let post: Post = element.entry().to_app_option()?.ok_or(WasmError::Guest(String::from("Malformed post")))?;

    Ok(post)
}
`;
