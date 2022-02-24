import { HappDefinition, ZomeDefinition } from '@holochain-scaffolding/definitions';
import { ScDirectory, ScNodeType } from '@source-craft/types';

import { zome } from '../zome';

import { dnaYaml } from './dna.yaml';

export function dna(happ: HappDefinition, dnaIndex: number, pathToBase: string): ScDirectory {
  const dna = happ.dnas[dnaIndex];

  const zomes: ScDirectory = {
    type: ScNodeType.Directory,
    children: {},
  };

  for (const [zomeIndex, zomeDef] of dna.zomes.entries()) {
    const z = zome(happ, dnaIndex, zomeIndex);
    zomes.children[zomeDef.name] = z;
  }

  return {
    type: ScNodeType.Directory,
    children: {
      workdir: {
        type: ScNodeType.Directory,
        children: {
          'dna.yaml': dnaYaml(happ, dnaIndex, pathToBase),
        },
      },
      zomes: zomes,
    },
  };
}
