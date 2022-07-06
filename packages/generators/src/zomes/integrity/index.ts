import { HappDefinition, CoordinatorZomeDefinition, IntegrityZomeDefinition } from '@holochain-scaffolding/definitions';
import { ScDirectory, ScNodeType } from '@source-craft/types';
import { snakeCase } from 'lodash-es';

import { integrityZomeCargoToml } from './Cargo.toml';
import { libRs } from './lib.rs';
import { generateEntryDef } from './definition';

export * from './definition';

export function integrityZomeCode(integrityZomeDefinition: IntegrityZomeDefinition, hdkVersion: string, hdiVersion: string): ScDirectory {
  const zomeDir: ScDirectory = {
    type: ScNodeType.Directory,
    children: {
      'lib.rs': libRs(integrityZomeDefinition),
    },
  };

  for (const entryDef of integrityZomeDefinition.entry_defs) {
    zomeDir.children[snakeCase(entryDef.typeDefinition.name)] = generateEntryDef(entryDef, hdkVersion, hdiVersion);
  }

  return zomeDir;
}

export function integrityZome(happ: HappDefinition, dnaIndex: number, zomeIndex: number, hdkVersion: string, hdiVersion: string): ScDirectory {
  const crateName = getIntegrityCrateName(happ, dnaIndex, zomeIndex);
  const dependingIntegrityZome = happ.dnas[dnaIndex].integrityZomes[zomeIndex];

  return {
    type: ScNodeType.Directory,
    children: {
      'Cargo.toml': integrityZomeCargoToml(crateName, '<AUTHOR>', hdiVersion),
      src: integrityZomeCode(dependingIntegrityZome, hdkVersion, hdiVersion),
    },
  };
}



export function getIntegrityCrateName(happ: HappDefinition, dnaIndex: number, zomeIndex: number): string {
  let thereIsAnotherZomeInAnotherDnaWithTheSameName = false;
  const zome = happ.dnas[dnaIndex].integrityZomes[zomeIndex];

  for (let i = 0; i < happ.dnas.length; i++) {
    const dna = happ.dnas[i];
    for (let j = 0; j < dna.integrityZomes.length; j++) {
      if (i !== dnaIndex || j !== zomeIndex) {
        if (dna.integrityZomes[j].name === zome.name) {
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
