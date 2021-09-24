use hdk::prelude::*;

#[hdk_entry(id = "post")]
pub struct Post(String);

entry_defs![
    Post::entry_def()
];

#[hdk_extern]
pub fn create_post(post: Post) -> ExternResult<EntryHash> {
    create_entry(&post)?;
    hash_entry(&post)
}

#[hdk_extern]
pub fn get_post(entry_hash: EntryHash) -> ExternResult<Post> {
    let element = get(entry_hash, GetOptions::default())?.ok_or(WasmError::Guest(String::from("Post not found")))?;

    let post: Post = element.entry().to_app_option()?.ok_or(WasmError::Guest(String::from("Malformed post")))?;

    Ok(post)
}
