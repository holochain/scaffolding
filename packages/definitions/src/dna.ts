import { newIntegrityZomeDef, IntegrityZomeDefinition, newCoordinatorZomeDef, CoordinatorZomeDefinition } from './zomes';

export interface DnaDefinition {
  name: string;
  integrityZomes: IntegrityZomeDefinition[];
  coordinatorZomes: CoordinatorZomeDefinition[];
}

export function newDnaDef(name = 'dna_0'): DnaDefinition {
  return {
    name,
    integrityZomes: [newIntegrityZomeDef()],
    coordinatorZomes: [newCoordinatorZomeDef()],
  };
}
