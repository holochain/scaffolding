import { HappDefinition, CoordinatorZomeDefinition, IntegrityZomeDefinition } from '@holochain-scaffolding/definitions';
import { snakeCase } from 'lodash-es';

export function getCoordinatorCrateName(happ: HappDefinition, dnaIndex: number, coordinatorZomeIndex: number): string {
  let thereIsAnotherZomeInAnotherDnaWithTheSameName = false;
  const coordinatorZome = happ.dnas[dnaIndex].coordinator_zomes[coordinatorZomeIndex];

  for (let i = 0; i < happ.dnas.length; i++) {
    const dna = happ.dnas[i];
    for (let j = 0; j < dna.coordinator_zomes.length; j++) {
      if (i !== dnaIndex || j !== coordinatorZomeIndex) {
        if (dna.coordinator_zomes[j].name === coordinatorZome.name) {
          thereIsAnotherZomeInAnotherDnaWithTheSameName = true;
        }
      }
    }
    for (let j = 0; j < dna.integrity_zomes.length; j++) {
      if (dna.integrity_zomes[j].name === coordinatorZome.name) {
        thereIsAnotherZomeInAnotherDnaWithTheSameName = true;
      }
    }
  }

  if (thereIsAnotherZomeInAnotherDnaWithTheSameName) {
    return `${snakeCase(happ.dnas[dnaIndex].name)}_${snakeCase(coordinatorZome.name)}`;
  } else {
    return snakeCase(coordinatorZome.name);
  }
}

export function getIntegrityCrateName(happ: HappDefinition, dnaIndex: number, integrityZomeIndex: number): string {
  let thereIsAnotherZomeInAnotherDnaWithTheSameName = false;
  const integrityZome = happ.dnas[dnaIndex].integrity_zomes[integrityZomeIndex];

  for (let i = 0; i < happ.dnas.length; i++) {
    const dna = happ.dnas[i];
    for (let j = 0; j < dna.coordinator_zomes.length; j++) {
      if (dna.coordinator_zomes[j].name === integrityZome.name) {
        thereIsAnotherZomeInAnotherDnaWithTheSameName = true;
      }
    }
    for (let j = 0; j < dna.integrity_zomes.length; j++) {
      if (i !== dnaIndex || j !== integrityZomeIndex) {
        if (dna.integrity_zomes[j].name === integrityZome.name) {
          thereIsAnotherZomeInAnotherDnaWithTheSameName = true;
        }
      }
    }
  }

  if (thereIsAnotherZomeInAnotherDnaWithTheSameName) {
    return `${snakeCase(happ.dnas[dnaIndex].name)}_${snakeCase(integrityZome.name)}`;
  } else {
    return snakeCase(integrityZome.name);
  }
}
