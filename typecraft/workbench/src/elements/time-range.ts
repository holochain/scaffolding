import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { html, LitElement } from 'lit';
import { CreateType, DetailType, TypeDefinition } from './type-definition';

export interface TimeRangeConfig {}

export class CreateTimeRange
  extends ScopedElementsMixin(LitElement)
  implements CreateType<TimeRange, TimeRangeConfig>
{
  configuration!: TimeRangeConfig;
  get value(): TimeRange {
    throw new Error('Method not implemented.');
  }
  render() {
    return html``;
  }
}
export class TimeRangeDetail
  extends LitElement
  implements DetailType<TimeRange, TimeRangeConfig>
{
  configuration!: TimeRangeConfig;
  value!: TimeRange;
}

export interface TimeRange {
  startTime: number;
  endTime: number;
}

export const timeRangeTypeDefinition: TypeDefinition<
  TimeRange,
  TimeRangeConfig
> = {
  name: 'TimeRange',
  description: '',

  create: [
    {
      element: CreateTimeRange,
      package: '',
      version: '',
    },
  ],
  detail: [
    {
      element: TimeRangeDetail,
      package: '',
      version: '',
    },
  ],
};
