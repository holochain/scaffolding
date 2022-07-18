import { EntryDefinition, newEntryDef } from './entry';

export interface IntegrityZomeDefinition {
  name: string;
  entry_defs: EntryDefinition[];
}

export interface CoordinatorZomeDefinition {
  name: string;
  dependencies: string[];
}

export const hdkTypes = ['AgentPubKey', 'EntryHash', 'ActionHash'];

export function newIntegrityZomeDef(name: string): IntegrityZomeDefinition {
  return {
    name,
    entry_defs: [newEntryDef()],
  };
}

export function newCoordinatorZomeDef(name: string, dependencies: string[]): CoordinatorZomeDefinition {
  return {
    name,
    dependencies,
  };
}
