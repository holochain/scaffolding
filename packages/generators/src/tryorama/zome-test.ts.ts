import { DnaDefinition, EntryDefinition, ZomeDefinition } from '@holochain/rad-definitions';
import { mergeStrings } from '../utils';
import toJsonSchema from 'to-json-schema';
import jsf from 'json-schema-faker';
jsf.option('alwaysFakeOptionals', true);
jsf.option('fillProperties', false);

export default (dna: DnaDefinition, zome: ZomeDefinition) => `
import { Orchestrator, Player, Cell } from "@holochain/tryorama";
import { config, installation, sleep } from '../utils';

export default (orchestrator: Orchestrator<any>) =>  {
  ${mergeStrings(zome.entry_defs.filter(e => e.create).map(entry_def => entryCrudTests(dna, zome, entry_def)))}
}

`;

export const entryCrudTests = (dna: DnaDefinition, zome: ZomeDefinition, entryDef: EntryDefinition) => `
  orchestrator.registerScenario("${zome.name} CRUD tests", async (s, t) => {
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

    const entryContents = ${JSON.stringify(entryDef.sample)};

    // Alice creates a ${entryDef.name}
    let create_output = await alice.call(
        "${zome.name}",
        "create_${entryDef.name}",
        entryContents
    );
    t.ok(create_output.header_hash);
    t.ok(create_output.entry_hash);

    await sleep(50);
    ${
      entryDef.read
        ? `
    // Bob gets the created ${entryDef.name}
    let entry = await bob.call("${zome.name}", "get_${entryDef.name}", create_output.entry_hash);
    t.deepEqual(entry, entryContents);
    `
        : ``
    }
    ${
      entryDef.update
        ? `
    // Alice updates the ${entryDef.name}
    let update_output = await alice.call(
      "${zome.name}",
      "update_${entryDef.name}",
      {
        original_header_hash: create_output.header_hash,
        updated_${entryDef.name}: ${JSON.stringify(generateAnotherSample(entryDef.sample), null, 2).replace(
            '\n',
            '\n        ',
          )}
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
    // Alice delete the ${entryDef.name}
    await alice.call(
      "${zome.name}",
      "delete_${entryDef.name}",
      create_output.header_hash
    );
    await sleep(50);

    ${
      entryDef.read
        ? `
    // Bob tries to get the deleted ${entryDef.name}, but he doesn't get it because it has been deleted
    let deletedEntry = await bob.call("${zome.name}", "get_${entryDef.name}", create_output.entry_hash);
    t.notOk(deletedEntry);`
        : ``
    }
      `
        : ``
    }
  });
`;

function generateAnotherSample(sample: any): any {
  let schema = toJsonSchema(sample);
  return jsf.generate(schema);
}
