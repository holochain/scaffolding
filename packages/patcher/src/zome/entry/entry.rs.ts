import { EntryDefinition } from '@holochain/rad-definitions';
import { PatcherFile, PatcherNodeType } from '@patcher/types';
import { run } from '../../typegen';
import { toTitleCase } from '../../utils';

export async function entryTypes(entryDef: EntryDefinition): Promise<PatcherFile> {
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

  const content = `use hdk::prelude::*;

#[hdk_entry(id = "${entryDef.name}")]
#[derive(Clone)]
${entryDefResult}`;

  return {
    type: PatcherNodeType.File,
    content,
  };
}
