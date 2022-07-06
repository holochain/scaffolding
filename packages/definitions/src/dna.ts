import { newZomeBundleDef, ZomeBundleDefinition } from './zomes';

export interface DnaDefinition {
  name: string;
  zomeBundles: ZomeBundleDefinition[];}

export function newDnaDef(name = 'dna_0'): DnaDefinition {
  return {
    name,
    zomeBundles: [newZomeBundleDef()],
  };
}
