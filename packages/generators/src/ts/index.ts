import { DnaDefinition, HappDefinition } from '@holochain/rad-definitions';
import { FileChanges, FileChangesType } from '../file-changes';
import { tsTypesForZome } from './types';

export async function generateTsTypes(happ: HappDefinition): Promise<FileChanges[]> {
  const promises = happ.dnas.map(dna => generateTsTypesForDna(dna));
  const dnaTsTypes = await Promise.all(promises);

  return [
    {
      type: FileChangesType.InDir,
      dirName: 'types',
      changes: [].concat(...dnaTsTypes),
    },
  ];
}

export async function generateTsTypesForDna(dna: DnaDefinition): Promise<FileChanges[]> {
  const promises = dna.zomes.map(
    async zome =>
      ({
        type: FileChangesType.Create,
        fileName: zome.name,
        content: await tsTypesForZome(zome),
      } as FileChanges),
  );

  const changes = await Promise.all(promises);

  return [
    {
      type: FileChangesType.InDir,
      dirName: dna.name,
      changes,
    },
  ];
}
