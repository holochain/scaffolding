import { EntryDefinition } from '@holochain/rad-definitions';
import { FileChanges, FileChangesType } from '../../file-changes';

import { generateEntryTypes } from './entry.rs';
import { generateEntryHandlers } from './handlers.rs';
import modRs from './mod.rs';

export * from './handlers.rs';
export * from './entry.rs';

export async function generateEntryDef(entryDef: EntryDefinition): Promise<FileChanges[]> {
  return [
    {
      type: FileChangesType.Create,
      fileName: `mod.rs`,
      content: modRs(),
    },
    {
      type: FileChangesType.Create,
      fileName: `handlers.rs`,
      content: generateEntryHandlers(entryDef),
    },
    {
      type: FileChangesType.Create,
      fileName: `entry.rs`,
      content: await generateEntryTypes(entryDef),
    },
  ];
}
