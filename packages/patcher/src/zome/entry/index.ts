import { EntryDefinition } from '@holochain-scaffolding/definitions';
import { PatcherDirectory, PatcherNodeType } from '@patcher/types';

import { entryTypes } from './entry.rs';
import { entryHandlers } from './handlers.rs';
import { modRs } from './mod.rs';

export * from './handlers.rs';
export * from './entry.rs';

export async function generateEntryDef(entryDef: EntryDefinition): Promise<PatcherDirectory> {
  return {
    type: PatcherNodeType.Directory,
    children: {
      'mod.rs': modRs(),
      'handlers.rs': entryHandlers(entryDef),
      'entry.rs': await entryTypes(entryDef),
    },
  };
}
