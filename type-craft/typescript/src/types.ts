import { NpmImport } from '@source-craft/npm';

export type VocabularyTypescriptGenerators = Record<string, TypescriptTypeGenerator>;

export interface TypescriptTypeGenerator {
  imports: NpmImport[];
  defineType: string;
  referenceType: string;
}
