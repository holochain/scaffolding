import { DnaDefinition, HappDefinition } from '@holochain/rad-definitions';

import { FileChanges, FileChangesType } from '../file-changes';
import packageJson from './package.json';
import tsconfig from './tsconfig.json';
import indexts from './index-ts';
import entryTests from './entry-test.ts';
import utils from './utils.ts';
import { ZomeDefinition } from '@holochain/rad-definitions';

export function generateTryorama(happ: HappDefinition): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'package.json',
      content: packageJson('0.4.6'),
    },
    {
      type: FileChangesType.Create,
      fileName: 'tsconfig.json',
      content: tsconfig(),
    },
    {
      type: FileChangesType.InDir,
      dirName: 'src',
      changes: [
        {
          type: FileChangesType.Create,
          fileName: 'index.ts',
          content: indexts(happ),
        },
        {
          type: FileChangesType.Create,
          fileName: 'utils.ts',
          content: utils(happ),
        },
        ...generateDnaTests(happ.dnas),
      ],
    },
  ];
}

function generateDnaTests(dnas: DnaDefinition[]): FileChanges[] {
  return dnas.map(dna => ({
    type: FileChangesType.InDir,
    dirName: dna.name,
    changes: dna.zomes.map(zome => generateZomeTests(dna, zome)),
  }));
}

function generateZomeTests(dna: DnaDefinition, zome: ZomeDefinition): FileChanges {
  return {
    type: FileChangesType.InDir,
    dirName: zome.name,
    changes: zome.entry_defs.map(entryDef => ({
      type: FileChangesType.Create,
      fileName: `${entryDef.name}.ts`,
      content: entryTests(dna, zome, entryDef),
    })),
  };
}
