import { EntryDefinition, holochainEntryRustTypeGenerator } from '@holochain-scaffolding/definitions';
import { ScFile } from '@source-craft/types';
import { generateRustTypesFile, VocabularyRustGenerators } from '@type-craft/rust';
import { happRustGenerators, happVocabulary } from '@holochain-scaffolding/vocabulary';
import { Vocabulary } from '@type-craft/vocabulary';

export function entryTypes(entryDef: EntryDefinition): ScFile {
  const typeDef = entryDef.typeDefinition;
  const vocabulary: Vocabulary = {
    ...happVocabulary,
    [typeDef.name]: typeDef,
  };

  const rustGenerators: VocabularyRustGenerators = {
    ...happRustGenerators,
    [typeDef.name]: holochainEntryRustTypeGenerator(typeDef.name, typeDef.fields),
  };

  return generateRustTypesFile(vocabulary, rustGenerators, [entryDef.typeDefinition]);
}
