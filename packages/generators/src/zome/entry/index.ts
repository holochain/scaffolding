import { EntryDefinition } from '@holochain-scaffolding/definitions';
import { ScDirectory, ScNodeType } from '@source-craft/types';

import { entryTypes } from './entry.rs';
import { entryHandlers } from './handlers.rs';
import { modRs } from './mod.rs';

export * from './handlers.rs';
export * from './entry.rs';

export function generateEntryDef(entryDef: EntryDefinition, hdkVersion: string, hdiVersion: string): ScDirectory {
  return {
    type: ScNodeType.Directory,
    children: {
      'mod.rs': modRs(),
      'handlers.rs': entryHandlers(entryDef),
      'entry.rs': entryTypes(entryDef, hdkVersion, hdiVersion),
    },
  };
}
