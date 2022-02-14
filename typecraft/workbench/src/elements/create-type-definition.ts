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
import { JsonSchemaForm } from './json-schema-form';
import { dateType } from './date';
import { FieldDefinition, TypeDefinition } from './type-definition';

const defaultTypes: TypeDefinition<any, any>[] = [
  /*   {
    name: 'Rating',
    configuration: {
      properties: {
        minRating: {
          description: 'Minimum rating allowed',
          type: 'number',
          default: 0,
          minimum: 0,
        },
        maxRating: {
          description: 'Maximum rating allowed',
          default: 5,
          type: 'number',
        },
      },
    },
    
  }, */
  dateType,
];

export class CreateTypeDefinition extends ScopedElementsMixin(LitElement) {
  @property({ type: Array }) typeDefs: TypeDefinition<any, any>[] =
    defaultTypes;

  @state()
  fields: Array<FieldDefinition<any>> = [
    {
      name: 'New Field',
      configuration: {},
      type: dateType,
    },
  ];

  @query('#type-name')
  typeNameField!: TextField;

  @query('#type-description')
  typeDescriptionField!: TextArea;

  get value(): TypeDefinition<any, any> {
    const name = this.typeNameField.value;
    const description = this.typeDescriptionField.value;
    console.log('heyy');
    return {
      name,
      description,
      fields: this.fields,
      create: [],
      detail: [],
    };
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
            @input=${(e: CustomEvent) =>
              (field.name = (e.target as TextField).value)}
            style="flex: 1; margin-right: 8px"
          ></mwc-textfield>
          <mwc-select
            .fixedMenuPosition=${true}
            outlined
            label="Field Type"
            @selected=${(e: CustomEvent) => {
              field.type = this.typeDefs[e.detail.index];
              this.requestUpdate();
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
          <mwc-icon-button
            icon="delete"
            @click=${() => {
              this.fields.splice(index, 1);
              this.requestUpdate();
              this.dispatchEvent(new Event('change'));
            }}
          ></mwc-icon-button>
        </div>

        <json-schema-form
          .schema=${field.type.configurationSchema}
          @change=${(e: Event) =>
            (field.configuration = (e.target as JsonSchemaForm).value)}
        ></json-schema-form>
      </div>
    `;
  }

  render() {
    return html`
      <div class="column" style="margin: 16px;">
        <mwc-textfield
          id="type-name"
          outlined
          label="Type Name"
          style="margin-top: 16px; width: 30em;"
        ></mwc-textfield>
        <mwc-textarea
          outlined
          id="type-description"
          label="Type Description"
          style="margin-top: 16px"
        ></mwc-textarea>

        <span style="margin-top: 24px;">Fields</span>

        ${this.fields.map((f, i) => this.renderField(f, i))}
        <div style="margin-top: 16px;">
          <mwc-button
            label="Add Field"
            @click=${(e: CustomEvent) =>
              (this.fields = [
                ...this.fields,
                {
                  name: 'New Field',
                  type: dateType,
                  configuration: {},
                },
              ])}
          ></mwc-button>
        </div>
      </div>
    `;
  }

  static get scopedElements() {
    return {
      'mwc-textfield': TextField,
      'mwc-textarea': TextArea,
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
