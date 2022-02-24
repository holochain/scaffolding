import { ZomeDefinition, DnaDefinition, HappDefinition } from '@holochain-scaffolding/definitions';
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
      'package.json': tryoramaPackageJson('0.4.10'),
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

    for (const zome of dna.zomes) {
      (dnatests.children[dna.name] as ScDirectory).children[zome.name] = zomeTests(dna, zome);
    }
  }

  return dnatests;
}

function zomeTests(dna: DnaDefinition, zome: ZomeDefinition): ScDirectory {
  const zometests: ScDirectory = {
    type: ScNodeType.Directory,
    children: {},
  };

  for (const entryDef of zome.entry_defs) {
    zometests.children[`${entryDef.typeDefinition.name}.ts`] = tryoramaEntryTest(dna, zome, entryDef);
  }

  return zometests;
}
