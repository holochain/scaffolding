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
import { DnaSource } from "@holochain/client";
import { pause, runScenario } from "@holochain/tryorama";
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
    const createOutput: any = await alice.cells[0].callZome({
      zome_name: "${coordinatorZome.name}",
      fn_name: "${createHandlerFnName(entryDef.typeDefinition.name)}",
      payload: createInput,
    });
    t.ok(createOutput.actionHash);  // test 1
    t.ok(createOutput.entryHash);   // test 2

    // Wait for the created entry to be propagated to the other node.
    await pause(100);

    ${
      entryDef.read
        ? `
    // Bob gets the created ${entryDef.typeDefinition.name}
    const readOutput: typeof createInput = await bob.cells[0].callZome({
      zome_name: "${coordinatorZome.name}",
      fn_name: "${readHandlerFnName(entryDef.typeDefinition.name)}",
      payload: createOutput.entryHash,
    });
    t.deepEqual(readOutput, createInput); // test 3
    `
        : ``
    }
    ${
      entryDef.update
        ? `
    // Alice updates the ${entryDef.typeDefinition.name}
    const contentUpdate = ${stringify(entryDef.typeDefinition.sample())}

    const updateInput = {
      originalActionHash: createOutput.actionHash,
      updated${upperFirst(camelCase(entryDef.typeDefinition.name))}: contentUpdate,
    }

    const updateOutput: any = await alice.cells[0].callZome({
      zome_name: "${coordinatorZome.name}",
      fn_name: "${updateHandlerFnName(entryDef.typeDefinition.name)}",
      payload: updateInput,
    });
    t.ok(updateOutput.actionHash);  // test 4
    t.ok(updateOutput.entryHash);   // test 5

    // Wait for the updated entry to be propagated to the other node.
    await pause(100);

      ${
        entryDef.read
          ? `
    // Bob gets the updated ${entryDef.typeDefinition.name}
    const readUpdatedOutput: typeof createInput = await bob.cells[0].callZome({
      zome_name: "${coordinatorZome.name}",
      fn_name: "${readHandlerFnName(entryDef.typeDefinition.name)}",
      payload: updateOutput.entryHash,
    });
    t.deepEqual(readUpdatedOutput, contentUpdate);  // test 6
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
      payload: createOutput.actionHash,
    })
    t.ok(deleteActionHash); // test 7

      ${
        entryDef.read
          ? `
    // Wait for the deletion action to be propagated to the other node.
    await pause(100);

    // Bob tries to get the deleted ${entryDef.typeDefinition.name}, but he doesn't get it because it has been deleted
    const readDeletedOutput = await bob.cells[0].callZome({
      zome_name: "${coordinatorZome.name}",
      fn_name: "${readHandlerFnName(entryDef.typeDefinition.name)}",
      payload: createOutput.entryHash,
    });
    t.notOk(readDeletedOutput); // test 8
`
          : ``
      }
    `
        : ``
    }
  });


`;
