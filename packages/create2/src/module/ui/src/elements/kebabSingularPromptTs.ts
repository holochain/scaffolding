import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const kebabSingularPromptTs = ({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural}: {moduleNamePluralTitleCase: string; _kebab: string; kebabPlural_: string; kebabSingular_: string; moduleNameTitleCase: string; moduleNamePlural: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `import { css, html, LitElement } from 'lit';
import { property, state } from 'lit/decorators.js';

import {
  Button,
  CircularProgress,
  TextField,
} from '@scoped-elements/material-web';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { contextProvided } from '@holochain-open-dev/context';
import { StoreSubscriber } from 'lit-svelte-stores';

import { sharedStyles } from './utils/shared-styles';
import { Create${moduleNameTitleCase} } from './create${_kebab}';
import { ${moduleNamePluralTitleCase}Store } from '../${kebabPlural_}store';
import { ${camelCase(moduleNamePlural)}StoreContext } from '../context';

/**
 * @element ${kebabSingular_}prompt
 * @slot hero - Will be displayed above the create${_kebab} form when the user is prompted with it
 */
export class ${moduleNameTitleCase}Prompt extends ScopedElementsMixin(LitElement) {
  /** Public attributes */

  /** Dependencies */

  /**
   * \`${moduleNamePluralTitleCase}Store\` that is requested via context.
   * Only set this property if you want to override the store requested via context.
   */
  @contextProvided({ context: ${camelCase(moduleNamePlural)}StoreContext })
  @property({ type: Object })
  store!: ${moduleNamePluralTitleCase}Store;

  /** Private properties */

  @state()
  private _loading = true;

  private _my${moduleNameTitleCase} = new StoreSubscriber(this, () => this.store?.my${moduleNameTitleCase});

  async firstUpdated() {
    await this.store.fetchMy${moduleNameTitleCase}();
    this._loading = false;
  }

  renderPrompt() {
    return html\` <div
      class="column"
      style="align-items: center; justify-content: center; flex: 1;"
    >
      \${this._loading
        ? html\`<mwc-circular-progress indeterminate></mwc-circular-progress>\`
        : html\` <div class="column" style="align-items: center;">
            <slot name="hero"></slot>
            <create${_kebab}></create${_kebab}>
          </div>\`}
    </div>\`;
  }

  render() {
    return html\`
      \${!this._loading && this._my${moduleNameTitleCase}.value
        ? html\`<slot></slot>\`
        : this.renderPrompt()}
    \`;
  }

  /**
   * @ignore
   */
  static get scopedElements() {
    return {
      'mwc-textfield': TextField,
      'mwc-button': Button,
      'mwc-circular-progress': CircularProgress,
      'create${_kebab}': Create${moduleNameTitleCase},
    };
  }

  static get styles() {
    return [
      sharedStyles,
      css\`
        :host {
          display: flex;
        }
      \`,
    ];
  }
}
`
});
    