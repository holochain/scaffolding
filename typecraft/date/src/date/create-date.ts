import { CreateElement, DetailElement } from '@typecraft/type-definition';
import { LitElement, html } from 'lit';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { DateConfig } from './types';

export class CreateDate extends ScopedElementsMixin(LitElement) implements CreateElement<number, DateConfig> {
  configuration!: DateConfig;
  get value(): number {
    return 0;
  }
  render() {
    return html``;
  }
}
