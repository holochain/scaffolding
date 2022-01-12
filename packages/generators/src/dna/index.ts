import { ZomeDefinition } from '@holochain/rad-definitions';
import { HappDefinition } from '@holochain/rad-definitions';
import { DnaDefinition } from '@holochain/rad-definitions';

import { FileChanges, FileChangesType } from '../file-changes';
import { generateZome } from '../zome';

import dnaYaml from './dna.yaml';

export function generateDnaYaml(happ: HappDefinition, dnaIndex: number, pathToBase: string): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'dna.yaml',
      content: dnaYaml(happ, dnaIndex, `${pathToBase}../`),
    },
  ];
}

export async function generateDna(happ: HappDefinition, dnaIndex: number, pathToBase: string): Promise<FileChanges[]> {
  const dna = happ.dnas[dnaIndex];
  const promises = dna.zomes.map(
    async (zome: ZomeDefinition, zomeIndex: number) =>
      ({
        type: FileChangesType.InDir,
        dirName: zome.name,
        changes: await generateZome(happ, dnaIndex, zomeIndex),
      } as FileChanges),
  );

  const zomeChanges: FileChanges[] = await Promise.all(promises);

  return [
    {
      type: FileChangesType.InDir,
      dirName: 'workdir',
      changes: generateDnaYaml(happ, dnaIndex, pathToBase),
    },
    {
      type: FileChangesType.InDir,
      dirName: 'zomes',
      changes: zomeChanges,
    },
  ];
}
