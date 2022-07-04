import { EntryDefinition, newEntryDef } from './entry';

export interface ZomeDefinition {
  name: string;
  entry_defs: EntryDefinition[];
}

export const hdkTypes = ['AgentPubKeyB64', 'EntryHashB64', 'ActionHashB64'];

export function newZomeDef(name = 'zome_0'): ZomeDefinition {
  return {
    entry_defs: [newEntryDef()],
    name,
  };
}
