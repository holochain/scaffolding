import { DnaDefinition, HappDefinition, ZomeDefinition } from '@holochain-scaffolding/definitions';
import { happVocabulary, happTsGenerators } from '@holochain-scaffolding/vocabulary';
import { ScDirectory, ScFile, ScNodeType } from '@source-craft/types';
import { generateTsTypesFile, VocabularyTypescriptGenerators } from '@type-craft/typescript';
import { Vocabulary } from '@type-craft/vocabulary';

export function generateTsTypesForHapp(happ: HappDefinition): ScDirectory {
  const types: Record<string, ScDirectory> = {};
  for (const dna of happ.dnas) {
    const dir = generateTsTypesForDna(dna);
    types[dna.name] = dir;
  }

  return {
    type: ScNodeType.Directory,
    children: types,
  };
}

export function generateTsTypesForDna(dna: DnaDefinition): ScDirectory {
  const files: Record<string, ScFile> = {};

  for (const zome of dna.zomes) {
    files[`${zome.name}.ts`] = tsTypesForZome(zome);
  }

  return {
    type: ScNodeType.Directory,
    children: files,
  };
}

export function tsTypesForZome(zome: ZomeDefinition): ScFile {
  const vocabulary: Vocabulary = {
    ...happVocabulary,
  };
  for (const entryDef of zome.entry_defs) {
    vocabulary[entryDef.typeDefinition.name] = entryDef.typeDefinition;
  }
  return generateTsTypesFile(
    vocabulary,
    happTsGenerators,
    zome.entry_defs.map(def => def.typeDefinition),
  );
}
