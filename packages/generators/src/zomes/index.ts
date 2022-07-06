import { HappDefinition } from '@holochain-scaffolding/definitions';
import { ScDirectory, ScNodeType } from '@source-craft/types';

import { integrityZome } from './integrity';
import { coordinatorZome } from './coordinator';

export * from './coordinator';
export * from './integrity';


// currently unused
export function zomeBundle(happ: HappDefinition, dnaIndex: number, zomeIndex: number, hdkVersion: string, hdiVersion: string): ScDirectory {

  const iz = integrityZome(happ, dnaIndex, zomeIndex, hdkVersion, hdiVersion);
  const cz = coordinatorZome(happ, dnaIndex, zomeIndex, hdkVersion);

  const zomeBundleDir: ScDirectory = {
    type: ScNodeType.Directory,
    children: {
      "coordinator": cz,
      "integrity": iz
    },
  };

  return zomeBundleDir;
};

