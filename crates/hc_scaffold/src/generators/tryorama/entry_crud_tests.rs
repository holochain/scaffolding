use std::path::PathBuf;

use crate::{cli::Crud, definitions::EntryDefinition};

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
import {{ dirname }} from 'node:path';
import {{ fileURLToPath }} from 'node:url';

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

fn common_setup(happ_bundle_path_from_tests_root: &PathBuf, dna_role_id: &String) -> String {
    format!(
        r#"
    // Construct proper paths for your app.
    // This assumes app bundle created by the `hc app pack` command.
    const testAppPath = process.cwd() + {:?};

    // Set up the array of DNAs to be installed, which only consists of the
    // test DNA referenced by path.
    const app = {{ appBundleSource: {{ path: testAppPath }} }};

    // Add 2 players with the test DNA to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithHappBundles([app, app]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();
    
    const alice_{}_cell = alice.cells.find(c => c.role_id === '{}');
    if (!alice_{}_cell) throw new Error("No cell for role id {} was found");

    const bob_{}_cell = bob.cells.find(c => c.role_id === '{}');
    if (!bob_{}_cell) throw new Error("No cell for role id {} was found");
    "#,
        happ_bundle_path_from_tests_root,
        dna_role_id,
        dna_role_id,
        dna_role_id,
        dna_role_id,
        dna_role_id,
        dna_role_id,
        dna_role_id,
        dna_role_id,
    )
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
        common_setup(happ_bundle_location_from_tests_root, dna_role_id),
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
        common_setup(happ_bundle_location_from_tests_root, dna_role_id),
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
        common_setup(happ_bundle_location_from_tests_root, dna_role_id),
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
        common_setup(happ_bundle_location_from_tests_root, dna_role_id),
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
/*
export const entryCrudTests = (
  dna: DnaDefinition,
  integrityZome: IntegrityZomeDefinition,
  coordinatorZome: CoordinatorZomeDefinition,
  entryDef: EntryDefinition,
) => `await runScenario(async scenario => {

    const dnas: DnaSource[] = [{path: ${camelCase(dna.name)}Dna }];

    const [alice, bob]  = await scenario.addPlayersWithHapps([dnas, dnas]);

    await scenario.shareAllAgents();

    const createInput = ${stringify(entryDef.typeDefinition.sample())};

    // Alice creates a ${entryDef.typeDefinition.name}
    const createActionHash: ActionHash = await alice.cells[0].callZome({
      zome_name: "${coordinatorZome.name}",
      fn_name: "${createHandlerFnName(entryDef.typeDefinition.name)}",
      payload: createInput,
    });
    t.ok(createActionHash);

    // Wait for the created entry to be propagated to the other node.
    await pause(100);

    ${
      entryDef.read
        ? `
    // Bob gets the created ${entryDef.typeDefinition.name}
    const createReadOutput: Record = await bob.cells[0].callZome({
      zome_name: "${coordinatorZome.name}",
      fn_name: "${readHandlerFnName(entryDef.typeDefinition.name)}",
      payload: createActionHash,
    });
    t.deepEqual(createInput, decode((createReadOutput.entry as any).Present.entry) as any);
    `
        : ``
    }
    ${
      entryDef.update
        ? `
    // Alice updates the ${entryDef.typeDefinition.name}
    const contentUpdate = ${stringify(entryDef.typeDefinition.sample())}

    const updateInput = {
      original_action_hash: createActionHash,
      updated_${snakeCase(entryDef.typeDefinition.name)}: contentUpdate,
    };

    const updateActionHash: ActionHash = await alice.cells[0].callZome({
      zome_name: "${coordinatorZome.name}",
      fn_name: "${updateHandlerFnName(entryDef.typeDefinition.name)}",
      payload: updateInput,
    });
    t.ok(updateActionHash);

    // Wait for the updated entry to be propagated to the other node.
    await pause(100);

      ${
        entryDef.read
          ? `
    // Bob gets the updated ${entryDef.typeDefinition.name}
    const readUpdatedOutput: Record = await bob.cells[0].callZome({
      zome_name: "${coordinatorZome.name}",
      fn_name: "${readHandlerFnName(entryDef.typeDefinition.name)}",
      payload: updateActionHash,
    });
    t.deepEqual(contentUpdate, decode((readUpdatedOutput.entry as any).Present.entry) as any);
`
          : ``
      }
    `
        : ``
    }
    ${
      entryDef.delete
        ? `
    // Alice deletes the ${entryDef.typeDefinition.name}
    const deleteActionHash = await alice.cells[0].callZome({
      zome_name: "${coordinatorZome.name}",
      fn_name: "${deleteHandlerFnName(entryDef.typeDefinition.name)}",
      payload: createActionHash,
    });
    t.ok(deleteActionHash);

      ${
        entryDef.read
          ? `
    // Wait for the deletion action to be propagated to the other node.
    await pause(100);

    // Bob tries to get the deleted ${entryDef.typeDefinition.name}, but he doesn't get it because it has been deleted
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "${coordinatorZome.name}",
      fn_name: "${readHandlerFnName(entryDef.typeDefinition.name)}",
      payload: createActionHash,
    });
    t.notOk(readDeletedOutput);
`
          : ``
      }
    `
        : ``
    }
  });


`;
" */
