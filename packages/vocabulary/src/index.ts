import { VocabularyRustGenerators } from '@type-craft/rust';
import { VocabularyTypescriptGenerators } from '@type-craft/typescript';
import { VocabularyElementsImportDeclarations } from '@type-craft/web-components';
import * as Title from '@type-craft/title';
import * as Content from '@type-craft/content';
import * as DateTime from '@type-craft/date-time';
import { Vocabulary } from '@type-craft/vocabulary';

import * as EntryHash from './hdk/entry-hash';
import * as ActionHash from './hdk/action-hash';
import * as AgentPubKey from './hdk/agent-pub-key';

export const happVocabulary: Vocabulary = {
  Title: Title.titleType,
  Content: Content.contentType,
  DateTime: DateTime.dateTimeType,
  EntryHash: EntryHash.type,
  ActionHash: ActionHash.type,
  AgentPubKey: AgentPubKey.type,
};

export function happRustGenerators(hdkVersion: string): VocabularyRustGenerators {
  return {
    Title: Title.rustGenerator,
    Content: Content.rustGenerator,
    DateTime: DateTime.rustGenerator,
    EntryHash: EntryHash.rustGenerator(hdkVersion),
    ActionHash: ActionHash.rustGenerator(hdkVersion),
    AgentPubKey: AgentPubKey.rustGenerator(hdkVersion),
  };
}

export const happTsGenerators: VocabularyTypescriptGenerators = {
  Title: Title.tsGenerator,
  Content: Content.tsGenerator,
  DateTime: DateTime.tsGenerator,
  EntryHash: EntryHash.tsGenerator,
  ActionHash: ActionHash.tsGenerator,
  AgentPubKey: AgentPubKey.tsGenerator,
};

export const elementsImports: VocabularyElementsImportDeclarations = {
  Title: Title.elementImports,
  Content: Content.elementImports,
  DateTime: DateTime.elementImports,
  EntryHash: EntryHash.elementsImports,
  ActionHash: ActionHash.elementsImports,
  AgentPubKey: AgentPubKey.elementsImports,
};
