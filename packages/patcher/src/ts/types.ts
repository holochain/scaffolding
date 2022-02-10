import { EntryDefinition, ZomeDefinition } from '@holochain/rad-definitions';
import { run } from '../typegen';
import { mergeStrings, toTitleCase } from '../utils';

export async function tsTypesForZome(zome: ZomeDefinition): Promise<string> {
  const promises = zome.entry_defs.map(tsTypeForEntry);

  const results = await Promise.all(promises);

  return mergeStrings(results);
}

export async function tsTypeForEntry(entryDef: EntryDefinition): Promise<string> {
  const type = await run(
    toTitleCase(entryDef.name),
    JSON.stringify(entryDef.sample),
    JSON.stringify({
      output_mode: 'typescript',
      import_style: 'assume_existing',
    }),
  );

  return `${type}
`;
}
