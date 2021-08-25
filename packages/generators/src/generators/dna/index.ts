import { FileChanges, FileChangesType } from '../../types';
import { camelToSnakeCase } from '../utils';
import { generateZome } from '../zome';

//@ts-ignore
import cargoToml from './Cargo.toml.hbs';
//@ts-ignore
import dnaYaml from './dna.yaml.hbs';

export function generateDnaCargoToml(zomeName: string): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'Cargo.toml',
      content: cargoToml({ zomeName }),
    },
  ];
}

export function generateDnaYaml(dnaName: string, zomeName: string): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'dna.yaml',
      content: dnaYaml({
        dnaName,
        zomeName: camelToSnakeCase(zomeName),
      }),
    },
  ];
}

export function generateDna(dnaName: string, zomeName: string): FileChanges[] {
  return [
    {
      type: FileChangesType.InDir,
      dirName: 'workdir',
      changes: generateDnaYaml(dnaName, zomeName),
    },
    ...generateDnaCargoToml(zomeName),
    {
      type: FileChangesType.InDir,
      dirName: 'zomes',
      changes: [
        {
          type: FileChangesType.InDir,
          dirName: zomeName,
          changes: generateZome(zomeName),
        },
      ],
    },
  ];
}
