import { DetailElement } from '@typecraft/type-definition';
import { LitElement } from 'lit';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { DateConfig } from './types';

export class ShowDate extends ScopedElementsMixin(LitElement) implements DetailElement<number, DateConfig> {
  configuration!: DateConfig;
  value!: number;
}
