import { DnaDefinition } from '@holochain/rad-definitions';

import { FileChanges, FileChangesType } from '../file-changes';
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

export async function generateDna(dna: DnaDefinition, pathToBase: string): Promise<FileChanges[]> {
  const promises = dna.zomes.map(
    async zome =>
      ({
        type: FileChangesType.InDir,
        dirName: zome.name,
        changes: await generateZome(zome),
      } as FileChanges),
  );

  const zomeChanges: FileChanges[] = await Promise.all(promises);

  return [
    {
      type: FileChangesType.InDir,
      dirName: 'workdir',
      changes: generateDnaYaml(dna, pathToBase),
    },
    {
      type: FileChangesType.InDir,
      dirName: 'zomes',
      changes: zomeChanges,
    },
  ];
}
