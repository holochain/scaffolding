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
import {
  FieldDefinition,
  TypeDefinition,
  Vocabulary,
} from '@type-craft/vocabulary';

import { JsonSchemaForm } from './json-schema-form';
import { UniqueFieldsController } from './unique-fields-controller';

export class CraftFields extends ScopedElementsMixin(LitElement) {
  @property({ type: Object }) vocabulary!: Vocabulary;

  @property()
  fields: Array<FieldDefinition<any>> = [];

  uniqueFieldsController = new UniqueFieldsController(
    this,
    () => this.uniqueFields
  );

  _fieldsCount = 0;

  get uniqueFields(): HTMLInputElement[] {
    return Array.from(
      this.shadowRoot!.querySelectorAll('.unique-field')
    ) as HTMLInputElement[];
  }

  get value(): Array<FieldDefinition<any>> {
    return this.fields;
  }

  getType(type: string): TypeDefinition<any, any> {
    return this.vocabulary[type];
  }

  renderField(field: FieldDefinition<any>, index: number) {
    const typeDefs = Object.values(this.vocabulary);
    return html`
      <div class="column" style="margin-top: 16px;">
        <div class="row" style="align-items: start">
          <mwc-textfield
            outlined
            label="Field Name"
            class="unique-field"
            required
            .value=${field.name}
            helper="Required and unique"
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
              field.type = typeDefs[e.detail.index].name;
              this.requestUpdate();
              this.dispatchEvent(new Event('change'));
            }}
          >
            ${typeDefs.map(
              t =>
                html`
                  <mwc-list-item
                    .value=${t.name}
                    .selected=${t.name === field.type}
                    >${t.name}</mwc-list-item
                  >
                `
            )}
          </mwc-select>

          ${this.getType(field.type).configurationSchema
            ? html`
                <div class="column" style="flex: 1; margin-top: -8px">
                  <span>Field Configuration</span>
                  <json-schema-form
                    .value=${field.configuration}
                    .schema=${this.getType(field.type).configurationSchema}
                    @change=${(e: Event) =>
                      (field.configuration = (
                        e.target as JsonSchemaForm
                      ).value)}
                  ></json-schema-form>
                </div>
              `
            : html``}

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
        ${this.fields.map((f, i) => this.renderField(f, i))}
        <div>
          <mwc-button
            label="Add Field"
            icon="add"
            @click=${(e: CustomEvent) => {
              this.fields = [
                ...this.fields,
                {
                  name: `new_field_${this._fieldsCount++}`,
                  type: Object.keys(this.vocabulary)[0],
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
