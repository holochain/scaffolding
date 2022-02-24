import { HappDefinition, ZomeDefinition } from '@holochain-scaffolding/definitions';
import { ScDirectory, ScFile, ScNodeType } from '@source-craft/types';

import { zomeCargoToml } from './Cargo.toml';
import { generateEntryDef } from './entry';
import { libRs } from './lib.rs';

export * from './entry';

export function zomeCode(zomeDefinition: ZomeDefinition): ScDirectory {
  const zomeDir: ScDirectory = {
    type: ScNodeType.Directory,
    children: {
      'lib.rs': libRs(zomeDefinition),
    },
  };

  for (const entryDef of zomeDefinition.entry_defs) {
    zomeDir.children[entryDef.typeDefinition.name] = generateEntryDef(entryDef);
  }

  return zomeDir;
}

export function zome(happ: HappDefinition, dnaIndex: number, zomeIndex: number): ScDirectory {
  const crateName = getCrateName(happ, dnaIndex, zomeIndex);
  const zome = happ.dnas[dnaIndex].zomes[zomeIndex];

  return {
    type: ScNodeType.Directory,
    children: {
      'Cargo.toml': zomeCargoToml(crateName, '<AUTHOR>'),
      src: zomeCode(zome),
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
    return `${happ.dnas[dnaIndex].name}_${zome.name}`;
  } else {
    return zome.name;
  }
}
