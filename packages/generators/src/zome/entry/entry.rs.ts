import { EntryDefinition } from '@holochain/rad-definitions';
import { run } from '../../typegen';
import { toTitleCase } from '../../utils';

export async function generateEntryTypes(entryDef: EntryDefinition): Promise<string> {
  const result = await run(
    toTitleCase(entryDef.name),
    JSON.stringify(entryDef.sample),
    JSON.stringify({
      derives: 'Clone, Serialize, Deserialize, Debug',
      import_style: 'assume_existing',
      property_name_format: 'camelCase',
    }),
  );

  const entryDefResult = result
    .split('\n')
    .slice(1)
    .join('\n');

  return `use hdk::prelude::*;

#[hdk_entry(id = "${entryDef.name}")]
#[derive(Clone)]
${entryDefResult}`;
}
