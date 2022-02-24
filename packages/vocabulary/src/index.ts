import { importDeclaration } from '@source-craft/web-apps';
import { VocabularyRustGenerators } from '@type-craft/rust';
import { VocabularyTypescriptGenerators } from '@type-craft/typescript';
import { VocabularyElementsImports } from '@type-craft/elements-imports';
import { Vocabulary } from '@type-craft/vocabulary';

export const happVocabulary: Vocabulary = {
  Title: {
    name: 'Title',
    description: '',
    sample: () => 'Entry Title',
  },
};

export const happRustGenerators: VocabularyRustGenerators = {
  Title: {
    defineType: '',
    imports: [],
    referenceType: 'String',
  },
};

export const happTsGenerators: VocabularyTypescriptGenerators = {
  Title: {
    defineType: '',
    imports: [],
    referenceType: 'string',
  },
};

export const renderersImports: VocabularyElementsImports = {
  Title: {
    create: {
      sideEffectImport: {
        importDeclaration: importDeclaration("import '@type-craft/description/create-description'"),
        packageName: '@type-craft/description',
        version: '^0.0.1',
      },
      tagName: 'create-description',
    },
    detail: {
      sideEffectImport: {
        importDeclaration: importDeclaration("import '@type-craft/description/description-detail'"),
        packageName: '@type-craft/description',
        version: '^0.0.1',
      },
      tagName: 'description-detail',
    },
  },
};
