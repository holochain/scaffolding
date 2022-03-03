import { DnaDefinition, EntryDefinition, ZomeDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { camelCase, snakeCase, upperFirst } from 'lodash-es';
import {
  createHandlerFnName,
  deleteHandlerFnName,
  readHandlerFnName,
  updateHandlerFnName,
} from '../zome/entry/handlers.rs';

export const tryoramaEntryTest = (dna: DnaDefinition, zome: ZomeDefinition, entryDef: EntryDefinition): ScFile => ({
  type: ScNodeType.File,
  content: `
import { Orchestrator, Player, Cell } from "@holochain/tryorama";
import { config, installation, sleep } from '../../utils';

export default (orchestrator: Orchestrator<any>) =>  {
  ${entryCrudTests(dna, zome, entryDef)}
}
`,
});

export const entryCrudTests = (dna: DnaDefinition, zome: ZomeDefinition, entryDef: EntryDefinition) => `
  orchestrator.registerScenario("${entryDef.typeDefinition.name} CRUD tests", async (s, t) => {
    // Declare two players using the previously specified config, nicknaming them "alice" and "bob"
    // note that the first argument to players is just an array conductor configs that that will
    // be used to spin up the conductor processes which are returned in a matching array.
    const [alice_player, bob_player]: Player[] = await s.players([config, config]);

    // install your happs into the conductors and destructuring the returned happ data using the same
    // array structure as you created in your installation array.
    const [[alice_happ]] = await alice_player.installAgentsHapps(installation);
    const [[bob_happ]] = await bob_player.installAgentsHapps(installation);

    await s.shareAllNodes([alice_player, bob_player]);

    const alice = alice_happ.cells.find(cell => cell.cellRole.includes('/${dna.name}.dna')) as Cell;
    const bob = bob_happ.cells.find(cell => cell.cellRole.includes('/${dna.name}.dna')) as Cell;

    const entryContents = ${JSON.stringify(entryDef.typeDefinition.sample(), null, 2)};

    // Alice creates a ${entryDef.typeDefinition.name}
    const create_output = await alice.call(
        "${zome.name}",
        "${createHandlerFnName(entryDef.typeDefinition.name)}",
        entryContents
    );
    t.ok(create_output.headerHash);
    t.ok(create_output.entryHash);

    await sleep(200);
    ${
      entryDef.read
        ? `
    // Bob gets the created ${entryDef.typeDefinition.name}
    const entry = await bob.call("${zome.name}", "${readHandlerFnName(
            entryDef.typeDefinition.name,
          )}", create_output.entryHash);
    t.deepEqual(entry, entryContents);
    `
        : ``
    }
    ${
      entryDef.update
        ? `
    // Alice updates the ${entryDef.typeDefinition.name}
    const update_output = await alice.call(
      "${zome.name}",
      "${updateHandlerFnName(entryDef.typeDefinition.name)}",
      {
        originalHeaderHash: create_output.headerHash,
        updated${upperFirst(camelCase(entryDef.typeDefinition.name))}: ${JSON.stringify(
            entryDef.typeDefinition.sample(),
            null,
            2,
          ).replace('\n', '\n        ')}
      }
    );
    t.ok(update_output.headerHash);
    t.ok(update_output.entryHash);
    await sleep(200);

      `
        : ``
    }
    ${
      entryDef.delete
        ? `
    // Alice delete the ${entryDef.typeDefinition.name}
    await alice.call(
      "${zome.name}",
      "${deleteHandlerFnName(entryDef.typeDefinition.name)}",
      create_output.headerHash
    );
    await sleep(200);

    ${
      entryDef.read
        ? `
    // Bob tries to get the deleted ${entryDef.typeDefinition.name}, but he doesn't get it because it has been deleted
    const deletedEntry = await bob.call("${zome.name}", "${readHandlerFnName(
            entryDef.typeDefinition.name,
          )}", create_output.entryHash);
    t.notOk(deletedEntry);`
        : ``
    }
      `
        : ``
    }
  });
`;
