import { DetailElement } from '@type-craft/vocabulary';
import { LitElement } from 'lit';
import { property } from 'lit/decorators.js';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { DateConfig } from './types';

export class ShowDate extends ScopedElementsMixin(LitElement) implements DetailElement<number, DateConfig> {
  fieldName: string;

  @property({ type: Boolean })
  relativeTime = false;

  value!: number;
}
