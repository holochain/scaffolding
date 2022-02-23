import { ProgrammingLanguages, TypeDefinition, TypeGenerator } from '@type-craft/vocabulary';
import { DateConfig } from './types';
import { CreateDate } from './create-date';
import { ShowDate } from './show-date';
import { rustGenerator, tsGenerator } from './generator';

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
  create: [
    {
      element: CreateDate,
      tagName: 'create-date',
      package: '@type-craft/date',
      version: '0.0.1',
      customImportDefiningCustomElement: '@type-craft/date/create-date',
    },
  ],
  detail: [
    {
      element: ShowDate,
      package: '@type-craft/date',
      tagName: 'show-date',
      version: '0.0.1',
      customImportDefiningCustomElement: '@type-craft/date/show-date',
    },
  ],

  sample: () => Date.now(),

  generators: {
    [ProgrammingLanguages.Typescript]: tsGenerator,
    [ProgrammingLanguages.Rust]: rustGenerator,
  },
};
