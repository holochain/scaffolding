import { VocabularyRustGenerators } from '@type-craft/rust';
import { VocabularyTypescriptGenerators } from '@type-craft/typescript';
import { VocabularyElementsImportDeclarations } from '@type-craft/web-components';
import * as Title from '@type-craft/title';
import * as Content from '@type-craft/content';
import * as DateTime from '@type-craft/date-time';
import { Vocabulary } from '@type-craft/vocabulary';

export const happVocabulary: Vocabulary = {
  Title: Title.titleType,
  Content: Content.contentType,
  DateTime: DateTime.dateTimeType,
};

export const happRustGenerators: VocabularyRustGenerators = {
  Title: Title.rustGenerator,
  Content: Content.rustGenerator,
  DateTime: DateTime.rustGenerator,
};

export const happTsGenerators: VocabularyTypescriptGenerators = {
  Title: Title.tsGenerator,
  Content: Content.tsGenerator,
  DateTime: DateTime.tsGenerator,
};

export const elementsImports: VocabularyElementsImportDeclarations = {
  Title: Title.elementImports,
  Content: Content.elementImports,
  DateTime: DateTime.elementImports,
};
