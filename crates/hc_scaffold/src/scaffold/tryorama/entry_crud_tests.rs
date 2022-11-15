use std::{collections::BTreeMap, path::PathBuf};

use convert_case::{Case, Casing};
use holochain_types::prelude::ZomeManifest;

use crate::scaffold::entry_type::{crud::Crud, definitions::EntryDefinition};

use super::utils::common_tests_setup;

pub fn entry_crud_tests(
    entry_definition: &EntryDefinition,
    dna_bundle_location_from_tests_root: &PathBuf,
    coordinator_zome: &String,
    crud: &Crud,
    link_original_to_each_update: bool,
    create_fns_of_entry_type_this_entry_type_depends_on: &BTreeMap<String, (ZomeManifest, String)>,
) -> String {
    let mut initial_test_file = format!(
        r#"
import test from 'node:test';
import assert from 'node:assert';

import {{ runScenario, pause }} from '@holochain/tryorama';
import {{ NewEntryAction, ActionHash, Record, DnaSource }} from '@holochain/client';
import {{ decode }} from '@msgpack/msgpack';

{}
"#,
        create_entry_test(
            entry_definition,
            dna_bundle_location_from_tests_root,
            coordinator_zome,
            create_fns_of_entry_type_this_entry_type_depends_on
        )
    );

    initial_test_file.push_str(
        read_entry_test(
            entry_definition,
            dna_bundle_location_from_tests_root,
            coordinator_zome,
        )
        .as_str(),
    );

    if crud.update {
        initial_test_file.push_str(
            update_entry_test(
                entry_definition,
                dna_bundle_location_from_tests_root,
                coordinator_zome,
                link_original_to_each_update,
            )
            .as_str(),
        );
    }

    if crud.delete {
        initial_test_file.push_str(
            delete_entry_test(
                entry_definition,
                dna_bundle_location_from_tests_root,
                coordinator_zome,
            )
            .as_str(),
        );
    }

    initial_test_file
}

fn create_depends_on_entries(
    create_fns_of_entry_type_this_entry_type_depends_on: &BTreeMap<String, (ZomeManifest, String)>,
) -> String {
    let mut initial_str = String::from("");

    // TODO: actually implement this
    for (_entry_type, (_zome, _fn_name)) in create_fns_of_entry_type_this_entry_type_depends_on {
        let create = format!(r#""#);
        initial_str.push_str(create.as_str());
    }

    initial_str
}

fn alice_create_entry(
    entry_definition: &EntryDefinition,
    coordinator_zome: &String,
    create_fns_of_entry_type_this_entry_type_depends_on: &BTreeMap<String, (ZomeManifest, String)>,
) -> String {
    format!(
        r#"{}
    const createInput = {};

    // Alice creates a {}
    const record: Record = await alice.cells[0].callZome({{
      zome_name: "{}",
      fn_name: "create_{}",
      payload: createInput,
    }});
    assert.ok(record);
"#,
        create_depends_on_entries(create_fns_of_entry_type_this_entry_type_depends_on),
        entry_definition.js_sample_object(),
        entry_definition.name,
        coordinator_zome,
        entry_definition.name
    )
}

pub fn create_entry_test(
    entry_definition: &EntryDefinition,
    happ_bundle_location_from_tests_root: &PathBuf,
    coordinator_zome: &String,
    create_fns_of_entry_type_this_entry_type_depends_on: &BTreeMap<String, (ZomeManifest, String)>,
) -> String {
    format!(
        r#"
test('create {}', async t => {{
  await runScenario(async scenario => {{
{}

{}
  }});
}});
"#,
        entry_definition.name,
        common_tests_setup(happ_bundle_location_from_tests_root),
        alice_create_entry(
            entry_definition,
            coordinator_zome,
            create_fns_of_entry_type_this_entry_type_depends_on
        )
    )
}

pub fn read_entry_test(
    entry_definition: &EntryDefinition,
    happ_bundle_location_from_tests_root: &PathBuf,
    coordinator_zome: &String,
) -> String {
    let hash_to_get_from = match entry_definition.fixed {
        true => format!("(record.signed_action.hashed.content as NewEntryAction).entry_hash"),
        false => format!("record.signed_action.hashed.hash"),
    };

    format!(
        r#"
test('create and read {}', async t => {{
  await runScenario(async scenario => {{
{}

    const createInput: any = {};

    // Alice creates a {}
    const record: Record = await alice.cells[0].callZome({{
      zome_name: "{}",
      fn_name: "create_{}",
      payload: createInput,
    }});
    assert.ok(record);
    
    // Wait for the created entry to be propagated to the other node.
    await pause(300);

    // Bob gets the created {}
    const createReadOutput: Record = await bob.cells[0].callZome({{
      zome_name: "{}",
      fn_name: "get_{}",
      payload: {hash_to_get_from},
    }});
    assert.deepEqual(createInput, decode((createReadOutput.entry as any).Present.entry) as any);
  }});
}});
"#,
        entry_definition.name,
        common_tests_setup(happ_bundle_location_from_tests_root),
        entry_definition.js_sample_object(),
        entry_definition.name,
        coordinator_zome,
        entry_definition.name,
        entry_definition.name,
        coordinator_zome,
        entry_definition.name
    )
}

pub fn update_entry_test(
    entry_definition: &EntryDefinition,
    happ_bundle_location_from_tests_root: &PathBuf,
    coordinator_zome: &String,
    link_original_to_each_update: bool,
) -> String {
    let read_after_update = |n: u32| {
        format!(
            r#"
    // Wait for the updated entry to be propagated to the other node.
    await pause(300);
        
    // Bob gets the updated {}
    const readUpdatedOutput{n}: Record = await bob.cells[0].callZome({{
      zome_name: "{}",
      fn_name: "get_{}",
      payload: updatedRecord.signed_action.hashed.hash,
    }});
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput{n}.entry as any).Present.entry) as any);
"#,
            entry_definition.name.to_case(Case::Snake),
            coordinator_zome,
            entry_definition.name.to_case(Case::Snake)
        )
    };

    let original_action_hash_field = match link_original_to_each_update {
        true => format!(
            r#"
      original_{}_hash: originalActionHash,"#,
            entry_definition.name.to_case(Case::Snake)
        ),
        false => String::from(""),
    };

    format!(
        r#"
test('create and update {}', async t => {{
  await runScenario(async scenario => {{
{}

    const createInput = {};

    // Alice creates a {}
    const record: Record = await alice.cells[0].callZome({{
      zome_name: "{}",
      fn_name: "create_{}",
      payload: createInput,
    }});
    assert.ok(record);
        
    const originalActionHash = record.signed_action.hashed.hash;
 
    // Alice updates the {}
    let contentUpdate: any = {};
    let updateInput = {{ {}
      previous_{}_hash: originalActionHash,
      updated_{}: contentUpdate,
    }};

    let updatedRecord: Record = await alice.cells[0].callZome({{
      zome_name: "{}",
      fn_name: "update_{}",
      payload: updateInput,
    }});
    assert.ok(updatedRecord);

{}

    // Alice updates the {} again
    contentUpdate = {};
    updateInput = {{ {}
      previous_{}_hash: updatedRecord.signed_action.hashed.hash,
      updated_{}: contentUpdate,
    }};

    updatedRecord = await alice.cells[0].callZome({{
      zome_name: "{}",
      fn_name: "update_{}",
      payload: updateInput,
    }});
    assert.ok(updatedRecord);

{}
  }});
}});
"#,
        entry_definition.name.to_case(Case::Snake),
        common_tests_setup(happ_bundle_location_from_tests_root),
        entry_definition.js_sample_object(),
        entry_definition.name.to_case(Case::Snake),
        coordinator_zome,
        entry_definition.name.to_case(Case::Snake),
        entry_definition.name.to_case(Case::Snake),
        entry_definition.js_sample_object(),
        original_action_hash_field,
        entry_definition.name.to_case(Case::Snake),
        entry_definition.name.to_case(Case::Snake),
        coordinator_zome,
        entry_definition.name.to_case(Case::Snake),
        read_after_update(0),
        entry_definition.name.to_case(Case::Snake),
        entry_definition.js_sample_object(),
        original_action_hash_field,
        entry_definition.name.to_case(Case::Snake),
        entry_definition.name.to_case(Case::Snake),
        coordinator_zome,
        entry_definition.name.to_case(Case::Snake),
        read_after_update(1)
    )
}

pub fn delete_entry_test(
    entry_definition: &EntryDefinition,
    happ_bundle_location_from_tests_root: &PathBuf,
    coordinator_zome: &String,
) -> String {
    let read_after_update = format!(
        r#"
    // Wait for the entry deletion to be propagated to the other node.
    await pause(300);
        
    // Bob tries to get the deleted {}
    const readDeletedOutput = await bob.cells[0].callZome({{
      zome_name: "{}",
      fn_name: "get_{}",
      payload: record.signed_action.hashed.hash,
    }});
    assert.equal(readDeletedOutput, undefined);
"#,
        entry_definition.name, coordinator_zome, entry_definition.name
    );
    format!(
        r#"
test('create and delete {}', async t => {{
  await runScenario(async scenario => {{
{}

    const createInput = {};

    // Alice creates a {}
    const record: Record = await alice.cells[0].callZome({{
      zome_name: "{}",
      fn_name: "create_{}",
      payload: createInput,
    }});
    assert.ok(record);
        
    // Alice deletes the {}
    const deleteActionHash = await alice.cells[0].callZome({{
      zome_name: "{}",
      fn_name: "delete_{}",
      payload: record.signed_action.hashed.hash,
    }});
    assert.ok(deleteActionHash);

{}
  }});
}});
"#,
        entry_definition.name,
        common_tests_setup(happ_bundle_location_from_tests_root),
        entry_definition.js_sample_object(),
        entry_definition.name,
        coordinator_zome,
        entry_definition.name,
        entry_definition.name,
        coordinator_zome,
        entry_definition.name,
        read_after_update
    )
}
