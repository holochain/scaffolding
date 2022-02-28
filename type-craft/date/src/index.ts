import { TypeDefinition } from '@type-craft/vocabulary';
import { TypeElementsImports } from '@type-craft/elements-imports';
import { importDeclaration } from '@source-craft/web-apps';
import { DateConfig } from './types';

export const dateType: TypeDefinition<number, DateConfig> = {
  name: 'Date',
  description: 'A point in time',

  configurationSchema: {
    properties: {
      relativeTime: {
        description: 'Display in relative time',
        type: 'boolean',
        default: false,
      },
    },
  },
  sample: () => Date.now(),
};

export const elementImports: TypeElementsImports = {
  create: {
    sideEffectImport: {
      importDeclaration: importDeclaration('@type-craft/date/create-date'),
      packageName: '@type-craft/date',
      version: '0.0.1',
    },
    tagName: 'create-date',
  },
  detail: {
    sideEffectImport: {
      importDeclaration: importDeclaration('@type-craft/date/show-date'),
      packageName: '@type-craft/date',
      version: '0.0.1',
    },
    tagName: 'show-date',
  },
};
