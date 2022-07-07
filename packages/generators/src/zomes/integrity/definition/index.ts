import { EntryDefinition } from '@holochain-scaffolding/definitions';
import { ScDirectory, ScNodeType } from '@source-craft/types';

import { entryDefinition } from './definition.rs';
import { modRs } from './mod.rs';

export * from './definition.rs';

export function generateEntryDef(entryDef: EntryDefinition, hdiVersion: string): ScDirectory {
  return {
    type: ScNodeType.Directory,
    children: {
      'mod.rs': modRs(),
      'definition.rs': entryDefinition(entryDef, hdiVersion),
    },
  };
}
