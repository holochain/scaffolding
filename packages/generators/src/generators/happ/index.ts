import { FileChanges, FileChangesType } from '../../types';
import { generateDna, generateDnaYaml } from '../dna';
import { generateNixFile } from '../nix';

//@ts-ignore
import happYaml from './happ.yaml.hbs';

export function generateHappYaml(happName: string, dnaName: string): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'happ.yaml',
      content: happYaml({
        happName,
        dnaName,
      }),
    },
  ];
}

export function generateHappWorkdir(happName: string, dnaName: string): FileChanges[] {
  return [
    {
      type: FileChangesType.InDir,
      dirName: 'workdir',
      changes: generateHappYaml(happName, dnaName),
    },
  ];
}

export function generateHapp(happName: string, dnaName: string, zomeName: string): FileChanges[] {
  return [
    {
      type: FileChangesType.InDir,
      dirName: happName,
      changes: [
        ...generateNixFile(),
        ...generateHappWorkdir(happName, dnaName),
        {
          type: FileChangesType.InDir,
          dirName: 'dnas',
          changes: [
            {
              type: FileChangesType.InDir,
              dirName: dnaName,
              changes: generateDna(dnaName, zomeName),
            },
          ],
        },
      ],
    },
  ];
}
