use std::path::PathBuf;

use crate::definitions::EntryDefinition;

pub fn entry_crud_tests(
    entry_definition: &EntryDefinition,
    happ_bundle_location_from_tests_root: &PathBuf,
    coordinator_zome: &String,
    create_function_name: &String,
) -> String {
    format!(
        r#"
import test from 'node:test';
import assert from 'node:assert';
import {{ runScenario, pause }} from '@holochain/tryorama';
import {{ dirname }} from 'node:path';
import {{ fileURLToPath }} from 'node:url';

{}
"#,
        create_entry_test(
            entry_definition,
            happ_bundle_location_from_tests_root,
            coordinator_zome,
            create_function_name
        )
    )
}

pub fn create_entry_test(
    entry_definition: &EntryDefinition,
    happ_bundle_location_from_tests_root: &PathBuf,
    coordinator_zome: &String,
    create_function_name: &String,
) -> String {
    format!(
        r#"
test('create {}', async t => {{
  await runScenario(async scenario => {{
    // Construct proper paths for your DNAs.
    // This assumes DNA files created by the `hc dna pack` command.
    const testAppPath = process.cwd() + '{:?}';

    // Set up the array of DNAs to be installed, which only consists of the
    // test DNA referenced by path.
    const app = [{{ appBundleSource: testAppPath }}];

    // Add 2 players with the test DNA to the Scenario. The returned players
    // can be destructured.
    const [alice, bob] = await scenario.addPlayersWithHappBundles([app, app]);

    // Shortcut peer discovery through gossip and register all agents in every
    // conductor of the scenario.
    await scenario.shareAllAgents();

    const createInput = {};

    // Alice creates a {}
    const createActionHash = await alice.cells[0].callZome({{
      zome_name: "{}",
      fn_name: "{}",
      payload: createInput,
    }});
    assert.ok(createActionHash);
  }});
}});"#,
        entry_definition.name,
        happ_bundle_location_from_tests_root,
        entry_definition.js_sample_object(),
        entry_definition.name,
        coordinator_zome,
        create_function_name
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
