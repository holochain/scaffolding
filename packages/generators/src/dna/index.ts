import { HappDefinition } from '@holochain-scaffolding/definitions';
import { ScDirectory, ScNodeType } from '@source-craft/types';

import { integrityZome, coordinatorZome } from '../zomes';

import { dnaYaml } from './dna.yaml';

export function dna(happ: HappDefinition, dnaIndex: number, pathToBase: string, hdkVersion: string, hdiVersion: string): ScDirectory {
  const dna = happ.dnas[dnaIndex];

  const zomeBundles: ScDirectory = {
    type: ScNodeType.Directory,
    children: {},
  };

  for (const [zomeIndex, coordinatorZomeDef] of dna.coordinatorZomes.entries()) {
    const iz = integrityZome(happ, dnaIndex, zomeIndex, hdkVersion, hdiVersion);
    const cz = coordinatorZome(happ, dnaIndex, zomeIndex, hdkVersion);
    zomeBundles.children[coordinatorZomeDef.name] = {
      type: ScNodeType.Directory,
      children: {
        "coordinator": cz,
        "integrity": iz
      }
    }
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
      zomes: zomeBundles,
    },
  };
}
