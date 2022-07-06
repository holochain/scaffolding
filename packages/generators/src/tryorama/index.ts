import { IntegrityZomeDefinition, DnaDefinition, HappDefinition, ZomeBundleDefinition } from '@holochain-scaffolding/definitions';
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
      'package.json': tryoramaPackageJson('0.5.8'),
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

    for (const [zomeBundleIndex, zomeBundle] of dna.zomeBundles.entries()) {
      (dnatests.children[dna.name] as ScDirectory).children[zomeBundle.name] = zomeTests(dna, zomeBundle);
    }
  }

  return dnatests;
}

function zomeTests(dna: DnaDefinition, zomeBundle: ZomeBundleDefinition): ScDirectory {
  const zometests: ScDirectory = {
    type: ScNodeType.Directory,
    children: {},
  };

  for (const entryDef of zomeBundle.integrityZome.entry_defs) {
    zometests.children[`${entryDef.typeDefinition.name}.ts`] = tryoramaEntryTest(dna, zomeBundle, entryDef);
  }

  return zometests;
}
