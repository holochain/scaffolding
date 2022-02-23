import { DetailElement } from '@type-craft/vocabulary';
import { LitElement } from 'lit';
import { property } from 'lit/decorators.js';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { DateConfig } from './types';

export class ShowDate extends ScopedElementsMixin(LitElement) implements DetailElement<number, DateConfig> {
  @property({ type: Boolean })
  relativeTime: boolean = false;

  value!: number;
}
