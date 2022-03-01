import { DnaDefinition, EntryDefinition, ZomeDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { snakeCase } from 'lodash-es';
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
    let create_output = await alice.call(
        "${zome.name}",
        "${createHandlerFnName(entryDef.typeDefinition.name)}",
        entryContents
    );
    t.ok(create_output.header_hash);
    t.ok(create_output.entry_hash);

    await sleep(50);
    ${
      entryDef.read
        ? `
    // Bob gets the created ${entryDef.typeDefinition.name}
    let entry = await bob.call("${zome.name}", "${readHandlerFnName(
            entryDef.typeDefinition.name,
          )}", create_output.entry_hash);
    t.deepEqual(entry, entryContents);
    `
        : ``
    }
    ${
      entryDef.update
        ? `
    // Alice updates the ${entryDef.typeDefinition.name}
    let update_output = await alice.call(
      "${zome.name}",
      "${updateHandlerFnName(entryDef.typeDefinition.name)}",
      {
        original_header_hash: create_output.header_hash,
        updated_${snakeCase(entryDef.typeDefinition.name)}: ${JSON.stringify(
            entryDef.typeDefinition.sample(),
            null,
            2,
          ).replace('\n', '\n        ')}
      }
    );
    t.ok(update_output.header_hash);
    t.ok(update_output.entry_hash);
    await sleep(50);

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
      create_output.header_hash
    );
    await sleep(50);

    ${
      entryDef.read
        ? `
    // Bob tries to get the deleted ${entryDef.typeDefinition.name}, but he doesn't get it because it has been deleted
    let deletedEntry = await bob.call("${zome.name}", "${readHandlerFnName(
            entryDef.typeDefinition.name,
          )}", create_output.entry_hash);
    t.notOk(deletedEntry);`
        : ``
    }
      `
        : ``
    }
  });
`;
