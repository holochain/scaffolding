import { TypeDefinition } from '@type-craft/vocabulary';
import { TypeElementsImports } from '@type-craft/elements-imports';
import { importDeclaration } from '@source-craft/web-apps';
export * from './generators';

export const titleType: TypeDefinition<string, {}> = {
  name: 'Title',
  description: 'Title of the object',

  sample: () => 'An amazing dinner',
};

export const elementImports: TypeElementsImports = {
  create: {
    sideEffectImport: {
      importDeclaration: importDeclaration('@type-craft/text/create-title'),
      packageName: '@type-craft/text',
      version: '0.0.1',
    },
    tagName: 'create-title',
  },
  detail: {
    sideEffectImport: {
      importDeclaration: importDeclaration('@type-craft/text/title-detail'),
      packageName: '@type-craft/text',
      version: '0.0.1',
    },
    tagName: 'title-detail',
  },
};
