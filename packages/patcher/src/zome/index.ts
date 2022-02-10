import { HappDefinition, ZomeDefinition } from '@holochain/rad-definitions';
import { PatcherDirectory, PatcherFile, PatcherNodeType } from '@patcher/types';

import { zomeCargoToml } from './Cargo.toml';
import { generateEntryDef } from './entry';
import { libRs } from './lib.rs';

export * from './entry';

export async function zomeCode(zomeDefinition: ZomeDefinition): Promise<PatcherDirectory> {
  const zomeDir: PatcherDirectory = {
    type: PatcherNodeType.Directory,
    children: {
      'lib.rs': libRs(zomeDefinition),
    },
  };

  for (const entryDef of zomeDefinition.entry_defs) {
    zomeDir.children[entryDef.name] = await generateEntryDef(entryDef);
  }

  return zomeDir;
}

export async function zome(happ: HappDefinition, dnaIndex: number, zomeIndex: number): Promise<PatcherDirectory> {
  const crateName = getCrateName(happ, dnaIndex, zomeIndex);
  const zome = happ.dnas[dnaIndex].zomes[zomeIndex];

  return {
    type: PatcherNodeType.Directory,
    children: {
      'Cargo.toml': zomeCargoToml(crateName, '<AUTHOR>'),
      src: await zomeCode(zome),
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
