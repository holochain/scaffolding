import { DnaDefinition, EntryDefinition, HappDefinition, ZomeDefinition } from '@holochain-scaffolding/definitions';
import { PatcherDirectory, PatcherFile, PatcherNodeType } from '@source-craft/types';
import { generateTypesFile, ProgrammingLanguages } from '@type-craft/vocabulary';

export function generateTsTypesForHapp(happ: HappDefinition): PatcherDirectory {
  const types: Record<string, PatcherDirectory> = {};
  for (const dna of happ.dnas) {
    const dir = generateTsTypesForDna(dna);
    types[dna.name] = dir;
  }

  return {
    type: PatcherNodeType.Directory,
    children: types,
  };
}

export function generateTsTypesForDna(dna: DnaDefinition): PatcherDirectory {
  const files: Record<string, PatcherFile> = {};

  for (const zome of dna.zomes) {
    const file: PatcherFile = {
      type: PatcherNodeType.File,
      content: tsTypesForZome(zome),
    };
    files[`${zome.name}.ts`] = file;
  }

  return {
    type: PatcherNodeType.Directory,
    children: files,
  };
}

export function tsTypesForZome(zome: ZomeDefinition): string {
  return generateTypesFile(zome.entry_defs.map(def => def.typeDefinition), ProgrammingLanguages.Typescript);
}
