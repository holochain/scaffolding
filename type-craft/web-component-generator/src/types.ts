import { NpmImport } from '@source-craft/npm';

export type VocabularyWebComponentGenerators = Record<string, WebComponentGenerator>;

export interface WebComponentGenerator {
  create: {
    imports: NpmImport[];
    tagName: string;
  };
  detail: {
    imports: NpmImport[];
    tagName: string;
  };
}
