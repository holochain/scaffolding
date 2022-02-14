import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { html, LitElement } from 'lit';
import { CreateType, DetailType, TypeDefinition } from './type-definition';

export interface DateConfig {
  relativeTime: boolean;
}

export class CreateDate
  extends ScopedElementsMixin(LitElement)
  implements CreateType<number, DateConfig>
{
  configuration!: DateConfig;
  get value(): number {
    throw new Error('Method not implemented.');
  }
  render() {
    return html``;
  }
}
export class DateDetail
  extends LitElement
  implements DetailType<number, DateConfig>
{
  configuration!: DateConfig;
  value!: number;
}

export const dateType: TypeDefinition<number, DateConfig> = {
  name: 'Date',
  description: 'A point in time',

  configurationSchema: {
    properties: {
      relativeTime: {
        description: 'Display in relative time',
        type: 'boolean',
        default: true,
      },
    },
  },
  create: [
    {
      element: CreateDate,
      package: '',
      version: '',
    },
  ],
  detail: [
    {
      element: DateDetail,
      package: '',
      version: '',
    },
  ],
};
