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
  coordinatorZomeIndex: number,
  hdkVersion: string,
): ScDirectory {
  const coordinatorCrateName = getCoordinatorCrateName(happ, dnaIndex, coordinatorZomeIndex);
  const integrityCrateName = getIntegrityCrateName(happ, dnaIndex, coordinatorZomeIndex);
  const coordinatorZome = happ.dnas[dnaIndex].coordinator_zomes[coordinatorZomeIndex];
  const integrityZome = happ.dnas[dnaIndex].integrity_zomes.find(iz => coordinatorZome.dependencies.includes(iz.name));

  return {
    type: ScNodeType.Directory,
    children: {
      'Cargo.toml': coordinatorZomeCargoToml(
        integrityZome.name,
        coordinatorCrateName,
        integrityCrateName,
        '<AUTHOR>',
        hdkVersion,
      ),
      src: coordinatorZomeCode(integrityZome, integrityCrateName),
    },
  };
}
