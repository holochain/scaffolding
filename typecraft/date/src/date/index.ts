import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { html, LitElement } from 'lit';
import { TypeDefinition } from '@typecraft/type-definition';
import { DateConfig } from './types';
import { CreateDate } from './create-date';
import { ShowDate } from './show-date';

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
      package: '@typecraft/date',
      version: '0.0.1',
      customImportDefiningCustomElement: '@typecraft/date/create-date',
    },
  ],
  detail: [
    {
      element: ShowDate,
      package: '@typecraft/date',
      version: '0.0.1',
      customImportDefiningCustomElement: '@typecraft/date/show-date',
    },
  ],
};
