import {
  CoordinatorZomeDefinition,
  IntegrityZomeDefinition,
  newCoordinatorZomeDef,
  newIntegrityZomeDef,
} from './zomes';

export interface DnaDefinition {
  name: string;
  integrity_zomes: IntegrityZomeDefinition[];
  coordinator_zomes: CoordinatorZomeDefinition[];
}

export function newDnaDef(name = 'dna_0'): DnaDefinition {
  return {
    name,
    integrity_zomes: [newIntegrityZomeDef('zome_0_integrity')],
    coordinator_zomes: [newCoordinatorZomeDef('zome_0', ['zome_0_integrity'])],
  };
}
