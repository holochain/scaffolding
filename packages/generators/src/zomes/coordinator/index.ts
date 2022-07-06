import { HappDefinition, CoordinatorZomeDefinition, IntegrityZomeDefinition } from '@holochain-scaffolding/definitions';
import { ScDirectory, ScNodeType } from '@source-craft/types';
import { snakeCase } from 'lodash-es';

import { coordinatorZomeCargoToml } from './Cargo.toml';
import { libRs } from './lib.rs';
import { entryHandlers } from './entry.rs';

export * from './entry.rs';

export function coordinatorZomeCode(dependingIntegrityZome: IntegrityZomeDefinition): ScDirectory {
  const zomeDir: ScDirectory = {
    type: ScNodeType.Directory,
    children: {
      'lib.rs': libRs(dependingIntegrityZome),
    },
  };

  for (const entryDef of dependingIntegrityZome.entry_defs) {
    zomeDir.children[snakeCase(entryDef.typeDefinition.name)] = entryHandlers(entryDef, dependingIntegrityZome.name);
  }

  return zomeDir;
}

export function coordinatorZome(happ: HappDefinition, dnaIndex: number, zomeIndex: number, hdkVersion: string): ScDirectory {
  const crateName = getCoordinatorCrateName(happ, dnaIndex, zomeIndex);
  const dependingIntegrityZome = happ.dnas[dnaIndex].integrityZomes[zomeIndex];

  return {
    type: ScNodeType.Directory,
    children: {
      'Cargo.toml': coordinatorZomeCargoToml(crateName, '<AUTHOR>', hdkVersion),
      src: coordinatorZomeCode(dependingIntegrityZome),
    },
  };
}



export function getCoordinatorCrateName(happ: HappDefinition, dnaIndex: number, zomeIndex: number): string {
  let thereIsAnotherZomeInAnotherDnaWithTheSameName = false;
  const zome = happ.dnas[dnaIndex].coordinatorZomes[zomeIndex];

  for (let i = 0; i < happ.dnas.length; i++) {
    const dna = happ.dnas[i];
    for (let j = 0; j < dna.coordinatorZomes.length; j++) {
      if (i !== dnaIndex || j !== zomeIndex) {
        if (dna.coordinatorZomes[j].name === zome.name) {
          thereIsAnotherZomeInAnotherDnaWithTheSameName = true;
        }
      }
    }
  }

  if (thereIsAnotherZomeInAnotherDnaWithTheSameName) {
    return `${snakeCase(happ.dnas[dnaIndex].name)}_${snakeCase(zome.name)}`;
  } else {
    return snakeCase(zome.name);
  }
}
