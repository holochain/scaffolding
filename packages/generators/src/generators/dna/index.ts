import { DnaDefinition, FileChanges, FileChangesType } from '../../types';
import { camelToSnakeCase } from '../utils';
import { generateZome } from '../zome';

import dnaYaml from './dna.yaml';

export function generateDnaYaml(dna: DnaDefinition, pathToBase: string): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'dna.yaml',
      content: dnaYaml(dna, `${pathToBase}../`),
    },
  ];
}

export function generateDna(dna: DnaDefinition, pathToBase: string): FileChanges[] {
  return [
    {
      type: FileChangesType.InDir,
      dirName: 'workdir',
      changes: generateDnaYaml(dna, pathToBase),
    },
    {
      type: FileChangesType.InDir,
      dirName: 'zomes',
      changes: dna.zomes.map(zome => ({
        type: FileChangesType.InDir,
        dirName: zome.name,
        changes: generateZome(zome),
      })),
    },
  ];
}
