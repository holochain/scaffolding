import { EntryDefinition, newEntryDef } from './entry';

export interface ZomeBundleDefinition {
  name: string;
  integrityZome: IntegrityZomeDefinition;
  coordinatorZome: CoordinatorZomeDefinition;
}

export interface IntegrityZomeDefinition {
  name: string;
  entry_defs: EntryDefinition[];
}

export interface CoordinatorZomeDefinition {
  name: string;
  dependencies: IntegrityZomeDefinition[];
}

export const hdkTypes = ['AgentPubKey', 'EntryHash', 'ActionHash'];

export function newZomeBundleDef(name = 'zome_0'): ZomeBundleDefinition {
  return {
    name,
    integrityZome: newIntegrityZomeDef(`${name}_integrity`),
    coordinatorZome: newCoordinatorZomeDef(name, [newIntegrityZomeDef(`${name}_integrity`)]),
  };
}

export function newIntegrityZomeDef(name: string): IntegrityZomeDefinition {
  return {
    name,
    entry_defs: [newEntryDef()],
  };
}

export function newCoordinatorZomeDef(name: string, dependencies): CoordinatorZomeDefinition {
  return {
    name,
    dependencies,
  };
}
