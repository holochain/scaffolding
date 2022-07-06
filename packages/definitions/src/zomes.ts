import { EntryDefinition, newEntryDef } from './entry';

export interface IntegrityZomeDefinition {
  name: string;
  entry_defs: EntryDefinition[];
}

export interface CoordinatorZomeDefinition {
  name: string;
  dependencies: IntegrityZomeDefinition[];
}

export const hdkTypes = ['AgentPubKeyB64', 'EntryHashB64', 'ActionHashB64'];

export function newIntegrityZomeDef(name = 'zome_0_integrity'): IntegrityZomeDefinition {
  return {
    entry_defs: [newEntryDef()],
    name,
  };
}

export function newCoordinatorZomeDef(name = 'zome_0', dependencies = [newIntegrityZomeDef()]): CoordinatorZomeDefinition {
  return {
    name,
    dependencies,
  };
}