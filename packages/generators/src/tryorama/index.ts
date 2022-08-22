import {
  IntegrityZomeDefinition,
  DnaDefinition,
  HappDefinition,
  CoordinatorZomeDefinition,
} from '@holochain-scaffolding/definitions';
import { ScDirectory, ScNodeType } from '@source-craft/types';

import { tryoramaPackageJson } from './package.json';
import { tryoramaTsConfig } from './tsconfig.json';
import { tryoramaIndexTs } from './index-ts';
import { tryoramaEntryTest } from './entry-test.ts';
import { tryoramaUtilsTs } from './utils.ts';

export function tryoramaTests(happ: HappDefinition): ScDirectory {
  const tests = dnaTests(happ.dnas);

  tests.children['index'];

  return {
    type: ScNodeType.Directory,
    children: {
      'package.json': tryoramaPackageJson('0.6.2'),
      'tsconfig.json': tryoramaTsConfig(),
      src: {
        type: ScNodeType.Directory,
        children: {
          ...tests.children,
          'index.ts': tryoramaIndexTs(happ),
          'utils.ts': tryoramaUtilsTs(happ),
        },
      },
    },
  };
}

function dnaTests(dnas: DnaDefinition[]): ScDirectory {
  const dnatests: ScDirectory = {
    type: ScNodeType.Directory,
    children: {},
  };

  for (const dna of dnas) {
    dnatests.children[dna.name] = {
      type: ScNodeType.Directory,
      children: {},
    };

    for (const [coordinatorZomeIndex, coordinatorZome] of dna.coordinator_zomes.entries()) {
      const integrityZome = dna.integrity_zomes.find(iz => coordinatorZome.dependencies.includes(iz.name));
      (dnatests.children[dna.name] as ScDirectory).children[coordinatorZome.name] = zomeTests(
        dna,
        coordinatorZome,
        integrityZome,
      );
    }
  }

  return dnatests;
}

function zomeTests(
  dna: DnaDefinition,
  coordinatorZome: CoordinatorZomeDefinition,
  integrityZome: IntegrityZomeDefinition,
): ScDirectory {
  const zometests: ScDirectory = {
    type: ScNodeType.Directory,
    children: {},
  };

  for (const entryDef of integrityZome.entry_defs) {
    zometests.children[`${entryDef.typeDefinition.name}.ts`] = tryoramaEntryTest(
      dna,
      integrityZome,
      coordinatorZome,
      entryDef,
    );
  }

  return zometests;
}
