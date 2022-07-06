import { HappDefinition, CoordinatorZomeDefinition, IntegrityZomeDefinition } from '@holochain-scaffolding/definitions';
import { snakeCase } from 'lodash-es';


export function getCoordinatorCrateName(happ: HappDefinition, dnaIndex: number, zomeBundleIndex: number): string {
  let thereIsAnotherZomeBundleInAnotherDnaWithTheSameName = false;
  const zomeBundle = happ.dnas[dnaIndex].zomeBundles[zomeBundleIndex];

  for (let i = 0; i < happ.dnas.length; i++) {
    const dna = happ.dnas[i];
    for (let j = 0; j < dna.zomeBundles.length; j++) {
      if (i !== dnaIndex || j !== zomeBundleIndex) {
        if (dna.zomeBundles[j].name === zomeBundle.name) {
          thereIsAnotherZomeBundleInAnotherDnaWithTheSameName = true;
        }
      }
    }
  }

  if (thereIsAnotherZomeBundleInAnotherDnaWithTheSameName) {
    return `${snakeCase(happ.dnas[dnaIndex].name)}_${snakeCase(zomeBundle.name)}`;
  } else {
    return snakeCase(zomeBundle.name);
  }
}


export function getIntegrityCrateName(happ: HappDefinition, dnaIndex: number, zomeBundleIndex: number): string {
  const coordinatorCrateName = getCoordinatorCrateName(happ, dnaIndex, zomeBundleIndex);
  return `${coordinatorCrateName}_integrity`;
}