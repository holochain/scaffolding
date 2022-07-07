import { HappDefinition, CoordinatorZomeDefinition, IntegrityZomeDefinition } from '@holochain-scaffolding/definitions';
import { ScDirectory, ScNodeType } from '@source-craft/types';
import { snakeCase } from 'lodash-es';

import { coordinatorZomeCargoToml } from './Cargo.toml';
import { libRs } from './lib.rs';
import { entryHandlers } from './entry.rs';
import { getCoordinatorCrateName, getIntegrityCrateName } from '../utils';

export * from './entry.rs';

export function coordinatorZomeCode(
  dependingIntegrityZome: IntegrityZomeDefinition,
  integrityCrateName: string,
): ScDirectory {
  const zomeDir: ScDirectory = {
    type: ScNodeType.Directory,
    children: {
      'lib.rs': libRs(dependingIntegrityZome),
    },
  };

  for (const entryDef of dependingIntegrityZome.entry_defs) {
    zomeDir.children[`${snakeCase(entryDef.typeDefinition.name)}.rs`] = entryHandlers(entryDef, integrityCrateName);
  }

  return zomeDir;
}

export function coordinatorZome(
  happ: HappDefinition,
  dnaIndex: number,
  zomeBundleIndex: number,
  hdkVersion: string,
): ScDirectory {
  const coordinatorCrateName = getCoordinatorCrateName(happ, dnaIndex, zomeBundleIndex);
  const integrityCrateName = getIntegrityCrateName(happ, dnaIndex, zomeBundleIndex);
  const zomeBundle = happ.dnas[dnaIndex].zomeBundles[zomeBundleIndex];

  return {
    type: ScNodeType.Directory,
    children: {
      'Cargo.toml': coordinatorZomeCargoToml(
        zomeBundle.name,
        coordinatorCrateName,
        integrityCrateName,
        '<AUTHOR>',
        hdkVersion,
      ),
      src: coordinatorZomeCode(zomeBundle.integrityZome, integrityCrateName),
    },
  };
}
