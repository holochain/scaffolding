import { html, css, LitElement, TemplateResult } from 'lit';
import { property, query, state } from 'lit/decorators.js';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import {
  Button,
  Card,
  IconButton,
  ListItem,
  Select,
  TextArea,
  TextField,
} from '@scoped-elements/material-web';
import { FieldDefinition, TypeDefinition } from '@typecraft/type-definition';

import { JsonSchemaForm } from './json-schema-form';
import { dateType } from '@typecraft/date';
import { defaultTypes } from './default-type-definitions';

export class CraftFields extends ScopedElementsMixin(LitElement) {
  @property({ type: Array }) typeDefs: TypeDefinition<any, any>[] =
    defaultTypes;

  @property()
  fields: Array<FieldDefinition<any>> = [
    {
      name: 'new_field',
      configuration: {},
      type: dateType,
    },
  ];

  get value(): Array<FieldDefinition<any>> {
    return this.fields;
  }

  getType(type: string): TypeDefinition<any, any> {
    return this.typeDefs.find(t => t.name === type) as TypeDefinition<any, any>;
  }

  renderField(field: FieldDefinition<any>, index: number) {
    return html`
      <div class="column" style="margin-top: 16px;">
        <div class="row" style="align-items: center">
          <mwc-textfield
            outlined
            label="Field Name"
            .value=${field.name}
            @input=${(e: CustomEvent) => {
              field.name = (e.target as TextField).value;
              this.dispatchEvent(new Event('change'));
            }}
            style="width: 12em; margin-right: 8px"
          ></mwc-textfield>
          <mwc-select
            .fixedMenuPosition=${true}
            outlined
            style="width: 12em; margin-right: 8px"
            label="Field Type"
            @selected=${(e: CustomEvent) => {
              field.type = this.typeDefs[e.detail.index];
              this.requestUpdate();
              this.dispatchEvent(new Event('change'));
            }}
          >
            ${this.typeDefs.map(
              t =>
                html`
                  <mwc-list-item
                    .value=${t.name}
                    .selected=${t.name === field.type.name}
                    >${t.name}</mwc-list-item
                  >
                `
            )}
          </mwc-select>

          <div class="column" style="flex: 1; margin-top: -8px">
            <span>Field Configuration</span>
            <json-schema-form
              .value=${field.configuration}
              .schema=${field.type.configurationSchema}
              @change=${(e: Event) =>
                (field.configuration = (e.target as JsonSchemaForm).value)}
            ></json-schema-form>
          </div>

          <mwc-icon-button
            icon="delete"
            .disabled=${this.fields.length < 2}
            @click=${() => {
              this.fields.splice(index, 1);
              this.requestUpdate();
              this.dispatchEvent(new Event('change'));
            }}
          ></mwc-icon-button>
        </div>
      </div>
    `;
  }

  render() {
    return html`
      <div class="column">
        <span style="font-size: 18px">Fields</span>

        ${this.fields.map((f, i) => this.renderField(f, i))}
        <div style="margin-top: 16px;">
          <mwc-button
            label="Add Field"
            icon="add"
            @click=${(e: CustomEvent) => {
              this.fields = [
                ...this.fields,
                {
                  name: 'new_field',
                  type: dateType,
                  configuration: {},
                },
              ];
              this.dispatchEvent(new Event('change'));
            }}
          ></mwc-button>
        </div>
      </div>
    `;
  }

  static get scopedElements() {
    return {
      'mwc-textfield': TextField,
      'mwc-button': Button,
      'mwc-select': Select,
      'mwc-list-item': ListItem,
      'mwc-card': Card,
      'mwc-icon-button': IconButton,
      'json-schema-form': JsonSchemaForm,
    };
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
