import { ZomeDefinition, DnaDefinition, HappDefinition } from '@holochain-scaffolding/definitions';
import { PatcherDirectory, PatcherNodeType } from '@patcher/types';

import { tryoramaPackageJson } from './package.json';
import { tryoramaTsConfig } from './tsconfig.json';
import { tryoramaIndexTs } from './index-ts';
import { tryoramaEntryTest } from './entry-test.ts';
import { tryoramaUtilsTs } from './utils.ts';

export function tryoramaTests(happ: HappDefinition): PatcherDirectory {
  const tests = dnaTests(happ.dnas);

  tests.children['index'];

  return {
    type: PatcherNodeType.Directory,
    children: {
      'package.json': tryoramaPackageJson('0.4.10'),
      'tsconfig.json': tryoramaTsConfig(),
      src: {
        type: PatcherNodeType.Directory,
        children: {
          ...tests.children,
          'index.ts': tryoramaIndexTs(happ),
          'utils.ts': tryoramaUtilsTs(happ),
        },
      },
    },
  };
}

function dnaTests(dnas: DnaDefinition[]): PatcherDirectory {
  const dnatests: PatcherDirectory = {
    type: PatcherNodeType.Directory,
    children: {},
  };

  for (const dna of dnas) {
    dnatests.children[dna.name] = {
      type: PatcherNodeType.Directory,
      children: {},
    };

    for (const zome of dna.zomes) {
      (dnatests.children[dna.name] as PatcherDirectory).children[zome.name] = zomeTests(dna, zome);
    }
  }

  return dnatests;
}

function zomeTests(dna: DnaDefinition, zome: ZomeDefinition): PatcherDirectory {
  const zometests: PatcherDirectory = {
    type: PatcherNodeType.Directory,
    children: {},
  };

  for (const entryDef of zome.entry_defs) {
    zometests.children[`${entryDef.name}.ts`] = tryoramaEntryTest(dna, zome, entryDef);
  }

  return zometests;
}
