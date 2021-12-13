import { FileChanges, FileChangesType } from '../../types/file-changes';
import { HappDefinition } from '../../types/happ';
import { DnaDefinition } from '../../types/dna';
import packageJson from './package.json';
import tsconfig from './tsconfig.json';
import indexts from './index.ts';
import zomeTests from './zome-test.ts';
import utils from './utils.ts';

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
          content: indexts(happ.dnas),
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
    changes: dna.zomes.map(zome => ({
      type: FileChangesType.Create,
      fileName: `${zome.name}.ts`,
      content: zomeTests(dna, zome),
    })),
  }));
}
