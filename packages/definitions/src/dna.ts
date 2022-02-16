import { newZomeDef, ZomeDefinition } from './zome';

export interface DnaDefinition {
  name: string;
  zomes: ZomeDefinition[];
}

export function newDnaDef(name: string = 'dna_0'): DnaDefinition {
  return {
    name,
    zomes: [newZomeDef()],
  };
}