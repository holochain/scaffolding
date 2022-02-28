import { CreateElement, DetailElement } from '@type-craft/vocabulary';
import { LitElement, html } from 'lit';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { DateConfig } from './types';
import { property } from 'lit/decorators.js';

export class CreateDate extends ScopedElementsMixin(LitElement) implements CreateElement<number, DateConfig> {
  fieldName: string;
  @property({ type: Boolean, attribute: 'relative-time' })
  relativeTime: boolean;

  get value(): number {
    return 0;
  }
  render() {
    return html``;
  }
}
