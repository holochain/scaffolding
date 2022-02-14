import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { Checkbox, Formfield, TextField } from '@scoped-elements/material-web';
import { JSONSchema7, JSONSchema7Definition } from 'json-schema';
import { html, LitElement } from 'lit';
import { property } from 'lit/decorators.js';
import { ref } from 'lit/directives/ref.js';

export class JsonSchemaForm extends ScopedElementsMixin(LitElement) {
  @property({ type: Object }) schema!: JSONSchema7;

  value: any = {};

  firstUpdated() {
    if (!this.schema.properties) return;
    for (const [name, prop] of Object.entries(this.schema.properties)) {
      this.value[name] = (prop as JSONSchema7).default;
    }
  }

  renderProperty(propertyName: string, property: JSONSchema7) {
    switch (property.type) {
      case 'boolean':
        return html` <mwc-formfield .label=${property.description}>
          <mwc-checkbox
            ${ref(
              el =>
                el &&
                property.default &&
                ((el as Checkbox).checked = property.default as boolean)
            )}
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
            ${ref(
              el =>
                el &&
                property.default &&
                ((el as TextField).value = `${property.default}`)
            )}
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
