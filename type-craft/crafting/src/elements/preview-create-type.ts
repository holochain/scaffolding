import { html, css, LitElement } from 'lit';
import { property, state } from 'lit/decorators.js';
import { unsafeHTML } from 'lit/directives/unsafe-html.js';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { TypeDefinition } from '@type-craft/vocabulary';

import kebabCase from 'lodash-es/kebabCase';

export class PreviewCreateType extends ScopedElementsMixin(LitElement) {
  @property()
  typeDef!: TypeDefinition<any, any>;

  firstUpdated() {
    if (this.typeDef.fields) {
      for (const field of this.typeDef.fields) {
        this.defineScopedElement(
          `create-${kebabCase(field.name)}`,
          field.type.create[0].element
        );
      }
    }
  }

  createHtml() {
    return `${this.typeDef.fields?.map(
      f => `<create-${kebabCase(f.name)}></create-${kebabCase(f.name)}>`
    )}`;
  }

  render() {
    return html`<div class="column">${unsafeHTML(this.createHtml())}</div>`;
  }

  static styles = css`
    .column {
      display: flex;
      flex-direction: column;
    }
    .row {
      display: flex;
      flex-direction: row;
    }
  `;
}
