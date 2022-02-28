import { DetailElement } from '@type-craft/vocabulary';
import { html, LitElement } from 'lit';
import { property } from 'lit/decorators.js';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';

export class TitleDetail extends ScopedElementsMixin(LitElement) implements DetailElement<string, {}> {
  @property()
  fieldName: string;

  @property()
  value!: string;

  render() {
    return html`
      <span>${this.value}</span>
    `;
  }
}
