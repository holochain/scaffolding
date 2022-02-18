import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const editKebabTs = ({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName}: {moduleNamePluralTitleCase: string; _kebab: string; kebabPlural_: string; kebabSingular_: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `import { contextProvided } from '@holochain-open-dev/context';
import { Dictionary } from '@holochain-open-dev/core-types';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import {
  Button,
  Card,
  Fab,
  IconButton,
  TextField,
} from '@scoped-elements/material-web';
import { SlAvatar } from '@scoped-elements/shoelace';
import { html, LitElement } from 'lit';
import { property, query, state } from 'lit/decorators.js';

import { ${moduleNamePluralTitleCase}Store } from '../${kebabPlural_}store';
import { ${camelCase(moduleNamePlural)}StoreContext } from '../context';
import { ${moduleNameTitleCase} } from '../types';
import { resizeAndExport } from './utils/image';
import { sharedStyles } from './utils/shared-styles';

/**
 * @element edit${_kebab}
 * @fires save${_kebab} - Fired when the save ${camelCase(moduleName)} button is clicked
 */
export class Edit${moduleNameTitleCase} extends ScopedElementsMixin(LitElement) {
  /**
   * The ${camelCase(moduleName)} to be edited.
   */
  @property({ type: Object })
  ${camelCase(moduleName)}: ${moduleNameTitleCase} | undefined;

  /**
   * Label for the save ${camelCase(moduleName)} button.
   */
  @property({ type: String, attribute: 'save${_kebab}-label' })
  save${moduleNameTitleCase}Label = 'Save ${moduleNameTitleCase}';

  /** Dependencies */

  /**
   * \`${moduleNamePluralTitleCase}Store\` that is requested via context.
   * Only set this property if you want to override the store requested via context.
   */
  @contextProvided({ context: ${camelCase(moduleNamePlural)}StoreContext })
  @property({ type: Object })
  store!: ${moduleNamePluralTitleCase}Store;

  /** Private properties */

  @query('#nickname-field')
  private _nicknameField!: TextField;

  private _existingUsernames: { [key: string]: boolean } = {};

  @query('#avatar-file-picker')
  private _avatarFilePicker!: HTMLInputElement;

  @state()
  private _avatar: string | undefined;

  firstUpdated() {
    this._avatar = this.${camelCase(moduleName)}?.fields['avatar'];

    this._nicknameField.validityTransform = (newValue: string) => {
      this.requestUpdate();
      if (newValue.length < this.store.config.minNicknameLength) {
        this._nicknameField.setCustomValidity(\`Nickname is too short\`);
        return {
          valid: false,
        };
      } else if (this._existingUsernames[newValue]) {
        this._nicknameField.setCustomValidity('This nickname already exists');
        return { valid: false };
      }

      return {
        valid: true,
      };
    };
  }

  onAvatarUploaded() {
    if (this._avatarFilePicker.files && this._avatarFilePicker.files[0]) {
      const reader = new FileReader();
      reader.onload = e => {
        const img = new Image();
        img.crossOrigin = 'anonymous';
        img.onload = () => {
          this._avatar = resizeAndExport(img);
          this._avatarFilePicker.value = '';
        };
        img.src = e.target?.result as string;
      };
      reader.readAsDataURL(this._avatarFilePicker.files[0]);
    }
  }

  avatarMode() {
    return this.store.config.avatarMode === 'avatar';
  }

  renderAvatar() {
    if (!this.avatarMode()) return html\`\`;
    return html\`
      <div
        style="width: 80px; height: 80px; justify-content: center;"
        class="row"
      >
        \${this._avatar
          ? html\`
              <div class="column" style="align-items: center; ">
                <sl-avatar
                  image="\${this._avatar}"
                  alt="Avatar"
                  style="margin-bottom: 4px; --size: 3.5rem;"
                  initials=""
                ></sl-avatar>
                <span
                  class="placeholder label"
                  style="cursor: pointer;   text-decoration: underline;"
                  @click=\${() => (this._avatar = undefined)}
                  >Clear</span
                >
              </div>
            \`
          : html\` <div class="column" style="align-items: center;">
              <mwc-fab
                icon="add"
                @click=\${() => this._avatarFilePicker.click()}
                style="margin-bottom: 4px;"
              ></mwc-fab>
              <span class="placeholder label">Avatar</span>
            </div>\`}
      </div>
    \`;
  }

  shouldSaveButtonBeEnabled() {
    if (!this._nicknameField) return false;
    if (!this._nicknameField.validity.valid) return false;
    if (this.avatarMode() && !this._avatar) return false;
    if (
      Object.values(this.getAdditionalTextFields()).find(t => !t.validity.valid)
    )
      return false;

    return true;
  }

  textfieldToFieldId(field: TextField): string {
    return field.id.split('-')[2];
  }

  getAdditionalFieldsValues(): Dictionary<string> {
    const textfields = this.getAdditionalTextFields();

    const values: Dictionary<string> = {};
    for (const [id, textfield] of Object.entries(textfields)) {
      values[id] = textfield.value;
    }

    return values;
  }

  getAdditionalTextFields(): Dictionary<TextField> {
    const textfields = Array.from(
      this.shadowRoot!.querySelectorAll('mwc-textfield')
    ).filter(f => f.id !== 'nickname-field') as TextField[];

    const fields: Dictionary<TextField> = {};
    for (const field of textfields) {
      const id = this.textfieldToFieldId(field);
      fields[id] = field;
    }
    return fields;
  }

  fireSave${moduleNameTitleCase}() {
    const nickname = this._nicknameField.value;

    const fields: Dictionary<string> = this.getAdditionalFieldsValues();
    if (this._avatar) {
      fields['avatar'] = this._avatar;
    }

    const ${camelCase(moduleName)}: ${moduleNameTitleCase} = {
      fields,
      nickname,
    };

    this.dispatchEvent(
      new CustomEvent('save${_kebab}', {
        detail: {
          ${camelCase(moduleName)},
        },
        bubbles: true,
        composed: true,
      })
    );
  }

  renderField(fieldName: string) {
    return html\`
      <mwc-textfield
        id="${kebabSingular_}field-\${fieldName}"
        outlined
        required
        autoValidate
        validationMessage="This field is required"
        .label=\${fieldName}
        .value=\${this.${camelCase(moduleName)}?.fields[fieldName] || ''}
        @input=\${() => this.requestUpdate()}
        style="margin-top: 8px;"
      ></mwc-textfield>
    \`;
  }

  render() {
    return html\`
      \${
        this.avatarMode()
          ? html\`<input
              type="file"
              id="avatar-file-picker"
              style="display: none;"
              @change=\${this.onAvatarUploaded}
            />\`
          : html\`\`
      }
        <div class="column">

          <div class="row" style="justify-content: center; margin-bottom: 8px; align-self: start;" >
            \${this.renderAvatar()}

            <mwc-textfield
              id="nickname-field"
              outlined
              label="Nickname"
              .value=\${this.${camelCase(moduleName)}?.nickname || ''}
              .helper=\${\`Min. \${this.store.config.minNicknameLength} characters\`}
              @input=\${() => this._nicknameField.reportValidity()}
              style="margin-left: 8px;"
            ></mwc-textfield>
          </div>

          \${this.store.config.additionalFields.map(field =>
            this.renderField(field)
          )}

          <mwc-button
            raised
            style="margin-top: 8px;"
            .disabled=\${!this.shouldSaveButtonBeEnabled()}
            .label=\${this.save${moduleNameTitleCase}Label}
            @click=\${() => this.fireSave${moduleNameTitleCase}()}
          ></mwc-button>
        </div>
      </mwc-card>
    \`;
  }

  /**
   * @ignore
   */
  static get scopedElements() {
    return {
      'mwc-textfield': TextField,
      'mwc-button': Button,
      'mwc-fab': Fab,
      'mwc-icon-button': IconButton,
      'sl-avatar': SlAvatar,
    };
  }

  static styles = [sharedStyles];
}
`
});
    