import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { Checkbox, Formfield, TextField } from '@scoped-elements/material-web';
import { JSONSchema7, JSONSchema7Definition } from 'json-schema';
import { html, LitElement } from 'lit';
import { property } from 'lit/decorators.js';
import { ref } from 'lit/directives/ref.js';

export class JsonSchemaForm extends ScopedElementsMixin(LitElement) {
  @property({ type: Object }) schema!: JSONSchema7;

  @property({ type: Object })
  value: any = {};

  firstUpdated() {
    if (!this.schema.properties) return;

    for (const [name, prop] of Object.entries(this.schema.properties)) {
      if (!this.value[name]) {
        this.value[name] = (prop as JSONSchema7).default;
      }
    }
  }

  renderProperty(propertyName: string, property: JSONSchema7) {
    switch (property.type) {
      case 'boolean':
        return html` <mwc-formfield .label=${property.description}>
          <mwc-checkbox
            .checked=${this.value[propertyName]}
            @change=${(e: Event) => {
              this.value[propertyName] = (e.target as Checkbox).checked;
              this.dispatchEvent(
                new Event('change', {
                  bubbles: true,
                  composed: true,
                })
              );
            }}
          ></mwc-checkbox
        ></mwc-formfield>`;
      case 'number':
        return html`
          <mwc-textfield
            type="number"
            outlined
            .value=${this.value[propertyName]}
            @input=${(e: Event) => {
              this.value[propertyName] = (e.target as TextField).value;
              this.dispatchEvent(new Event('change'));
            }}
            .label=${property.description}
            .min=${property.minimum}
            .max=${property.maximum}
          ></mwc-textfield>
        `;
    }

    return html``;
  }

  render() {
    if (!this.schema.properties) return html``;

    return html`
      ${Object.entries(this.schema.properties).map(
        ([name, p]) => typeof p === 'object' && this.renderProperty(name, p)
      )}
    `;
  }

  static get scopedElements() {
    return {
      'mwc-textfield': TextField,
      'mwc-checkbox': Checkbox,
      'mwc-formfield': Formfield,
    };
  }
}
