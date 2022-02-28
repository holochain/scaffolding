import { VocabularyRustGenerators } from '@type-craft/rust';
import { VocabularyTypescriptGenerators } from '@type-craft/typescript';
import { VocabularyElementsImports } from '@type-craft/elements-imports';
import { TitleType } from '@type-craft/text';
import { Vocabulary } from '@type-craft/vocabulary';

export const happVocabulary: Vocabulary = {
  Title: TitleType.titleType,
};

export const happRustGenerators: VocabularyRustGenerators = {
  Title: TitleType.rustGenerator,
};

export const happTsGenerators: VocabularyTypescriptGenerators = {
  Title: TitleType.tsGenerator,
};

export const renderersImports: VocabularyElementsImports = {
  Title: TitleType.elementImports
};
