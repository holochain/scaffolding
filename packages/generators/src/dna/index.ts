import { HappDefinition } from '@holochain-scaffolding/definitions';
import { ScDirectory, ScNodeType } from '@source-craft/types';

import { integrityZome, coordinatorZome } from '../zomes';

import { dnaYaml } from './dna.yaml';

export function dna(
  happ: HappDefinition,
  dnaIndex: number,
  pathToBase: string,
  hdkVersion: string,
  hdiVersion: string,
): ScDirectory {
  const dna = happ.dnas[dnaIndex];

  const integrity_zomes: ScDirectory = { type: ScNodeType.Directory, children: {} };
  const coordinator_zomes: ScDirectory = { type: ScNodeType.Directory, children: {} };

  for (const [zomeIndex, zomeBundleDef] of dna.zomeBundles.entries()) {
    const iz = integrityZome(happ, dnaIndex, zomeIndex, hdiVersion);
    const cz = coordinatorZome(happ, dnaIndex, zomeIndex, hdkVersion);
    integrity_zomes.children[zomeBundleDef.name] = iz;
    coordinator_zomes.children[zomeBundleDef.name] = cz;
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
      integrity_zomes,
      coordinator_zomes,
    },
  };
}
