import { CreateElement } from '@type-craft/vocabulary';
import { LitElement, html } from 'lit';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { TextField } from '@scoped-elements/material-web';
import { property, query } from 'lit/decorators.js';

export class CreateTitle extends ScopedElementsMixin(LitElement) implements CreateElement<string, {}> {
  @property()
  fieldName: string;

  @query('#title-field')
  titleField!: TextField;

  get value(): string {
    return this.titleField.value;
  }

  render() {
    return html`
      <mwc-textfield
        id="title-field"
        .label=${this.fieldName}
        @input=${() => this.dispatchEvent(new Event('change'))}
      ></mwc-textfield>
    `;
  }

  get scopedElements() {
    return { 'mwc-textfield': TextField };
  }
}
