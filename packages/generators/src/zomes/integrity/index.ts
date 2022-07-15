import { HappDefinition, CoordinatorZomeDefinition, IntegrityZomeDefinition } from '@holochain-scaffolding/definitions';
import { ScDirectory, ScNodeType } from '@source-craft/types';
import { snakeCase } from 'lodash-es';

import { integrityZomeCargoToml } from './Cargo.toml';
import { libRs } from './lib.rs';
import { generateEntryDef } from './definition';
import { getIntegrityCrateName } from '../utils';

export * from './definition';

export function integrityZomeCode(integrityZomeDefinition: IntegrityZomeDefinition, hdiVersion: string): ScDirectory {
  const zomeDir: ScDirectory = {
    type: ScNodeType.Directory,
    children: {
      'lib.rs': libRs(integrityZomeDefinition),
    },
  };

  for (const entryDef of integrityZomeDefinition.entry_defs) {
    zomeDir.children[snakeCase(entryDef.typeDefinition.name)] = generateEntryDef(entryDef, hdiVersion);
  }

  return zomeDir;
}

export function integrityZome(
  happ: HappDefinition,
  dnaIndex: number,
  zomeBundleIndex: number,
  hdiVersion: string,
): ScDirectory {
  const crateName = getIntegrityCrateName(happ, dnaIndex, zomeBundleIndex);
  const integrityZome = happ.dnas[dnaIndex].zomeBundles[zomeBundleIndex].integrityZome;

  return {
    type: ScNodeType.Directory,
    children: {
      'Cargo.toml': integrityZomeCargoToml(crateName, '<AUTHOR>', hdiVersion),
      src: integrityZomeCode(integrityZome, hdiVersion),
    },
  };
}
