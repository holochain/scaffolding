import { HappDefinition } from '@holochain-scaffolding/definitions';
import { ScDirectory, ScNodeType } from '@source-craft/types';

import { integrityZome } from '../zomes/integrity';
import { coordinatorZome } from '../zomes/coordinator';

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

  for (const [integrityZomeIndex, integrityZomeDef] of dna.integrity_zomes.entries()) {
    const iz = integrityZome(happ, dnaIndex, integrityZomeIndex, hdiVersion);
    integrity_zomes.children[integrityZomeDef.name] = iz;
  }
  for (const [coordinatorZomeIndex, coordinatorZomeDef] of dna.coordinator_zomes.entries()) {
    const cz = coordinatorZome(happ, dnaIndex, coordinatorZomeIndex, hdkVersion);
    coordinator_zomes.children[coordinatorZomeDef.name] = cz;
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
