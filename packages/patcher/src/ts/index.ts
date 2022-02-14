import { DnaDefinition, HappDefinition } from '@holochain-scaffolding/definitions';
import { PatcherDirectory, PatcherFile, PatcherNodeType } from '@patcher/types';

import { tsTypesForZome } from './types';

export async function generateTsTypes(happ: HappDefinition): Promise<PatcherDirectory> {
  const types: Record<string, PatcherDirectory> = {};
  for (const dna of happ.dnas) {
    const dir = await generateTsTypesForDna(dna);
    types[dna.name] = dir;
  }

  return {
    type: PatcherNodeType.Directory,
    children: types,
  };
}

export async function generateTsTypesForDna(dna: DnaDefinition): Promise<PatcherDirectory> {
  const files: Record<string, PatcherFile> = {};

  for (const zome of dna.zomes) {
    const file: PatcherFile = {
      type: PatcherNodeType.File,
      content: await tsTypesForZome(zome),
    };
    files[`${zome.name}.ts`] = file;
  }

  return {
    type: PatcherNodeType.Directory,
    children: files,
  };
}
