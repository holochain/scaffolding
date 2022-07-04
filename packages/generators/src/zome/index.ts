import { HappDefinition, ZomeDefinition } from '@holochain-scaffolding/definitions';
import { ScDirectory, ScNodeType } from '@source-craft/types';
import { snakeCase } from 'lodash-es';

import { zomeCargoToml } from './Cargo.toml';
import { generateEntryDef } from './entry';
import { libRs } from './lib.rs';

export * from './entry';

export function zomeCode(zomeDefinition: ZomeDefinition, hdkVersion: string, hdiVersion: string): ScDirectory {
  const zomeDir: ScDirectory = {
    type: ScNodeType.Directory,
    children: {
      'lib.rs': libRs(zomeDefinition),
    },
  };

  for (const entryDef of zomeDefinition.entry_defs) {
    zomeDir.children[snakeCase(entryDef.typeDefinition.name)] = generateEntryDef(entryDef, hdkVersion, hdiVersion);
  }

  return zomeDir;
}

export function zome(happ: HappDefinition, dnaIndex: number, zomeIndex: number, hdkVersion: string, hdiVersion: string): ScDirectory {
  const crateName = getCrateName(happ, dnaIndex, zomeIndex);
  const zome = happ.dnas[dnaIndex].zomes[zomeIndex];

  return {
    type: ScNodeType.Directory,
    children: {
      'Cargo.toml': zomeCargoToml(crateName, '<AUTHOR>', hdkVersion, hdiVersion),
      src: zomeCode(zome, hdkVersion, hdiVersion),
    },
  };
}

export function getCrateName(happ: HappDefinition, dnaIndex: number, zomeIndex: number): string {
  let thereIsAnotherZomeInAnotherDnaWithTheSameName = false;
  const zome = happ.dnas[dnaIndex].zomes[zomeIndex];

  for (let i = 0; i < happ.dnas.length; i++) {
    const dna = happ.dnas[i];
    for (let j = 0; j < dna.zomes.length; j++) {
      if (i !== dnaIndex || j !== zomeIndex) {
        if (dna.zomes[j].name === zome.name) {
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
