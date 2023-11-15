use hdk::prelude::*;
use hello_world_integrity::*;

#[hdk_extern]
pub fn hello_world(message: String) -> ExternResult<ActionHash> {

    // commit the Hello message
    let action_hash = create_entry(&EntryTypes::Hello(Hello{message}))?;

    // link it to an anchor for later retrieval
    let path = Path::from("hellos");
    create_link(
        path.path_entry_hash()?,
        action_hash.clone(),
        LinkTypes::AllHellos,
        (),
    )?;
    Ok(action_hash)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HelloOutput {
    pub message: String,
    pub author: AgentPubKey
}

#[hdk_extern]
pub fn get_hellos(_: ()) -> ExternResult<Vec<HelloOutput>> {

    // get all of the hellos linked to the anchor
    let path = Path::from("hellos");
    let links = get_links(path.path_entry_hash()?, LinkTypes::AllHellos, None)?;
    let get_input: Vec<GetInput> = links
        .into_iter()
        .map(|link| Ok(GetInput::new(
            link.target.into_action_hash().ok_or(wasm_error!(WasmErrorInner::Guest(String::from("No action hash associated with link"))))?.into(),
            GetOptions::default(),
        )))
        .collect::<ExternResult<Vec<GetInput>>>()?;

    // load the records for all the links
    let records = HDK.with(|hdk| hdk.borrow().get(get_input))?;
    let records: Vec<Record> = records.into_iter().filter_map(|r| r).collect();

    // convert the records into a usefull struct for the UI
    let mut hellos = Vec::new();
    for r in records {
        let maybe_hello: Option<Hello> = r.entry.to_app_option().map_err(|e| wasm_error!(e))?;
        if let Some(hello) = maybe_hello  {
            hellos.push(
                HelloOutput {
                    message:hello.message,
                    author: r.action().author().clone(),
                }
            )
        }
    }
    Ok(hellos)
}
