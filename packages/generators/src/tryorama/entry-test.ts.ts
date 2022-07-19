import {
  CoordinatorZomeDefinition,
  DnaDefinition,
  IntegrityZomeDefinition,
  EntryDefinition,
} from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { camelCase, snakeCase, upperFirst } from 'lodash-es';
import { json } from 'json-es6';
import {
  createHandlerFnName,
  deleteHandlerFnName,
  readHandlerFnName,
  updateHandlerFnName,
} from '../zomes/coordinator/entry.rs';

export const tryoramaEntryTest = (
  dna: DnaDefinition,
  integrityZome: IntegrityZomeDefinition,
  coordinatorZome: CoordinatorZomeDefinition,
  entryDef: EntryDefinition,
): ScFile => ({
  type: ScNodeType.File,
  content: `
import { DnaSource, Record, ActionHash } from "@holochain/client";
import { pause, runScenario } from "@holochain/tryorama";
import { decode } from '@msgpack/msgpack';
import pkg from 'tape-promise/tape';
const { test } = pkg;

import { ${camelCase(dna.name)}Dna } from  "../../utils";


export default () => test("${entryDef.typeDefinition.name} CRUD tests", async (t) => {
  ${entryCrudTests(dna, integrityZome, coordinatorZome, entryDef)}
});
`,
});

function replacer(key, value) {
  if (ArrayBuffer.isView(value)) {
    return `Buffer.from(new Uint8Array([${value}]))`;
  } else {
    return value;
  }
}

function stringify(sample): string {
  const s = JSON.stringify(sample, replacer, 2);
  return s.replace(/"(Buffer\.from\(new Uint8Array\(\[[0-9]*(?:,[0-9]*)*\]\)\))"/gm, '$1');
}

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
