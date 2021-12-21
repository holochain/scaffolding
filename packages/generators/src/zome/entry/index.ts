import { EntryDefinition } from '@holochain/rad-definitions';
import { FileChanges, FileChangesType } from '../../file-changes';

import { generateEntryTypes } from './entry.rs';
import handlersRs from './handlers.rs';
import modRs from './mod.rs';

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
      content: handlersRs(entryDef),
    },
    {
      type: FileChangesType.Create,
      fileName: `entry.rs`,
      content: await generateEntryTypes(entryDef),
    },
  ];
}
