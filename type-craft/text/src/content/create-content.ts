import { CreateElement } from '@type-craft/vocabulary';
import { LitElement, html } from 'lit';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { TextArea } from '@scoped-elements/material-web';
import { property, query } from 'lit/decorators.js';

export class CreateContent extends ScopedElementsMixin(LitElement) implements CreateElement<string, {}> {
  @property()
  fieldName: string;

  @query('#content-field')
  contentField!: TextArea;

  get value(): string {
    return this.contentField.value;
  }

  render() {
    return html`
      <mwc-textarea
        id="content-field"
        .label=${this.fieldName}
        @input=${() => this.dispatchEvent(new Event('change'))}
      ></mwc-textarea>
    `;
  }

  get scopedElements() {
    return { 'mwc-textarea': TextArea };
  }
}
