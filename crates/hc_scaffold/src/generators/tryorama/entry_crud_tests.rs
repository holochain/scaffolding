use std::path::PathBuf;

use crate::{cli::Crud, definitions::EntryDefinition};

use super::utils::common_tests_setup;

pub fn entry_crud_tests(
    entry_definition: &EntryDefinition,
    happ_bundle_location_from_tests_root: &PathBuf,
    dna_role_id: &String,
    coordinator_zome: &String,
    crud: &Crud,
) -> String {
    let mut initial_test_file = format!(
        r#"
import test from 'node:test';
import assert from 'node:assert';

import {{ runScenario, pause }} from '@holochain/tryorama';
import {{ ActionHash, Record }} from '@holochain/client';
import {{ decode }} from '@msgpack/msgpack';

{}
"#,
        create_entry_test(
            entry_definition,
            happ_bundle_location_from_tests_root,
            dna_role_id,
            coordinator_zome,
        )
    );

    if crud.read {
        initial_test_file.push_str(
            read_entry_test(
                entry_definition,
                happ_bundle_location_from_tests_root,
                dna_role_id,
                coordinator_zome,
            )
            .as_str(),
        )
    }

    if crud.update {
        initial_test_file.push_str(
            update_entry_test(
                entry_definition,
                happ_bundle_location_from_tests_root,
                dna_role_id,
                coordinator_zome,
                crud.read,
            )
            .as_str(),
        )
    }

    if crud.delete {
        initial_test_file.push_str(
            delete_entry_test(
                entry_definition,
                happ_bundle_location_from_tests_root,
                dna_role_id,
                coordinator_zome,
                crud.read,
            )
            .as_str(),
        )
    }

    initial_test_file
}

pub fn create_entry_test(
    entry_definition: &EntryDefinition,
    happ_bundle_location_from_tests_root: &PathBuf,
    dna_role_id: &String,
    coordinator_zome: &String,
) -> String {
    format!(
        r#"
test('create {}', async t => {{
  await runScenario(async scenario => {{
{}

    const createInput = {};

    // Alice creates a {}
    const createActionHash = await alice_{}_cell.callZome({{
      zome_name: "{}",
      fn_name: "create_{}",
      payload: createInput,
    }});
    assert.ok(createActionHash);
  }});
}});"#,
        entry_definition.name,
        common_tests_setup(happ_bundle_location_from_tests_root, dna_role_id),
        entry_definition.js_sample_object(),
        entry_definition.name,
        dna_role_id,
        coordinator_zome,
        entry_definition.name
    )
}

pub fn read_entry_test(
    entry_definition: &EntryDefinition,
    happ_bundle_location_from_tests_root: &PathBuf,
    dna_role_id: &String,
    coordinator_zome: &String,
) -> String {
    format!(
        r#"
test('create and read {}', async t => {{
  await runScenario(async scenario => {{
{}

    const createInput: any = {};

    // Alice creates a {}
    const createActionHash = await alice_{}_cell.callZome({{
      zome_name: "{}",
      fn_name: "create_{}",
      payload: createInput,
    }});
    assert.ok(createActionHash);
    
    // Wait for the created entry to be propagated to the other node.
    await pause(300);

    // Bob gets the created {}
    const createReadOutput: Record = await bob_{}_cell.callZome({{
      zome_name: "{}",
      fn_name: "get_{}",
      payload: createActionHash,
    }});
    assert.deepEqual(createInput, decode((createReadOutput.entry as any).Present.entry) as any);
  }});
}});"#,
        entry_definition.name,
        common_tests_setup(happ_bundle_location_from_tests_root, dna_role_id),
        entry_definition.js_sample_object(),
        entry_definition.name,
        dna_role_id,
        coordinator_zome,
        entry_definition.name,
        entry_definition.name,
        dna_role_id,
        coordinator_zome,
        entry_definition.name
    )
}

pub fn update_entry_test(
    entry_definition: &EntryDefinition,
    happ_bundle_location_from_tests_root: &PathBuf,
    dna_role_id: &String,
    coordinator_zome: &String,
    read_after_update: bool,
) -> String {
    let maybe_read = match read_after_update {
        false => String::from(""),
        true => format!(
            r#"
    // Wait for the updated entry to be propagated to the other node.
    await pause(300);
        
    // Bob gets the updated {}
    const readUpdatedOutput: Record = await bob_{}_cell.callZome({{
      zome_name: "{}",
      fn_name: "get_{}",
      payload: updateActionHash,
    }});
    assert.deepEqual(contentUpdate, decode((readUpdatedOutput.entry as any).Present.entry) as any);
"#,
            entry_definition.name, dna_role_id, coordinator_zome, entry_definition.name
        ),
    };

    format!(
        r#"
test('create and update {}', async t => {{
  await runScenario(async scenario => {{
{}

    const createInput = {};

    // Alice creates a {}
    const createActionHash = await alice_{}_cell.callZome({{
      zome_name: "{}",
      fn_name: "create_{}",
      payload: createInput,
    }});
    assert.ok(createActionHash);
 
    // Alice updates the {}
    const contentUpdate: any = {};

    const updateInput = {{
      original_action_hash: createActionHash,
      updated_{}: contentUpdate,
    }};

    const updateActionHash: ActionHash = await alice_{}_cell.callZome({{
      zome_name: "{}",
      fn_name: "update_{}",
      payload: updateInput,
    }});
    assert.ok(updateActionHash);

{}
  }});
}});"#,
        entry_definition.name,
        common_tests_setup(happ_bundle_location_from_tests_root, dna_role_id),
        entry_definition.js_sample_object(),
        entry_definition.name,
        dna_role_id,
        coordinator_zome,
        entry_definition.name,
        entry_definition.name,
        entry_definition.js_sample_object(),
        entry_definition.name,
        dna_role_id,
        coordinator_zome,
        entry_definition.name,
        maybe_read
    )
}

pub fn delete_entry_test(
    entry_definition: &EntryDefinition,
    happ_bundle_location_from_tests_root: &PathBuf,
    dna_role_id: &String,
    coordinator_zome: &String,
    read_after_delete: bool,
) -> String {
    let maybe_read = match read_after_delete {
        false => String::from(""),
        true => format!(
            r#"
    // Wait for the entry deletion to be propagated to the other node.
    await pause(300);
        
    // Bob tries to get the deleted {}
    const readDeletedOutput = await bob_{}_cell.callZome({{
      zome_name: "{}",
      fn_name: "get_{}",
      payload: createActionHash,
    }});
    assert.equal(readDeletedOutput, undefined);
"#,
            entry_definition.name, dna_role_id, coordinator_zome, entry_definition.name
        ),
    };
    format!(
        r#"
test('create and delete {}', async t => {{
  await runScenario(async scenario => {{
{}

    const createInput = {};

    // Alice creates a {}
    const createActionHash = await alice_{}_cell.callZome({{
      zome_name: "{}",
      fn_name: "create_{}",
      payload: createInput,
    }});
    assert.ok(createActionHash);
        
    // Alice deletes the {}
    const deleteActionHash = await alice_{}_cell.callZome({{
      zome_name: "{}",
      fn_name: "delete_{}",
      payload: createActionHash,
    }});
    assert.ok(deleteActionHash);

{}
  }});
}});"#,
        entry_definition.name,
        common_tests_setup(happ_bundle_location_from_tests_root, dna_role_id),
        entry_definition.js_sample_object(),
        entry_definition.name,
        dna_role_id,
        coordinator_zome,
        entry_definition.name,
        entry_definition.name,
        dna_role_id,
        coordinator_zome,
        entry_definition.name,
        maybe_read
    )
}
