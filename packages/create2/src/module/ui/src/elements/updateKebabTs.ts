import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const updateKebabTs = ({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName}: {moduleNamePluralTitleCase: string; _kebab: string; kebabPlural_: string; kebabSingular_: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `import { html, LitElement } from 'lit';
import { query, property, state } from 'lit/decorators.js';
import { contextProvided } from '@holochain-open-dev/context';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { Dictionary } from '@holochain-open-dev/core-types';
import {
  TextField,
  Button,
  Card,
  IconButton,
  Fab,
  CircularProgress,
} from '@scoped-elements/material-web';
import { SlAvatar } from '@scoped-elements/shoelace';

import { sharedStyles } from './utils/shared-styles';
import { ${moduleNamePluralTitleCase}Store } from '../${kebabPlural_}store';
import { ${camelCase(moduleNamePlural)}StoreContext } from '../context';
import { resizeAndExport } from './utils/image';
import { Edit${moduleNameTitleCase} } from './edit${_kebab}';
import { ${moduleNameTitleCase} } from '../types';
import { StoreSubscriber } from 'lit-svelte-stores';

/**
 * @element update${_kebab}
 * @fires ${kebabSingular_}updated - Fired after the ${camelCase(moduleName)} has been created. Detail will have this shape: { ${camelCase(moduleName)}: { nickname, fields } }
 */
export class Update${moduleNameTitleCase} extends ScopedElementsMixin(LitElement) {
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

  async update${moduleNameTitleCase}(${camelCase(moduleName)}: ${moduleNameTitleCase}) {
    await this.store.update${moduleNameTitleCase}(${camelCase(moduleName)});

    this.dispatchEvent(
      new CustomEvent('${kebabSingular_}updated', {
        detail: {
          ${camelCase(moduleName)},
        },
        bubbles: true,
        composed: true,
      })
    );
  }

  render() {
    if (this._loading)
      return html\`<div
        class="column"
        style="align-items: center; justify-content: center; flex: 1;"
      >
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      </div>\`;

    return html\`
      <edit${_kebab}
        .${camelCase(moduleName)}=\${this._my${moduleNameTitleCase}.value}
        save${_kebab}-label="Update ${moduleNameTitleCase}"
        @save${_kebab}=\${(e: CustomEvent) =>
          this.update${moduleNameTitleCase}(e.detail.${camelCase(moduleName)})}
      ></edit${_kebab}>
    \`;
  }

  /**
   * @ignore
   */
  static get scopedElements() {
    return {
      'mwc-circular-progress': CircularProgress,
      'edit${_kebab}': Edit${moduleNameTitleCase},
      'mwc-card': Card,
    };
  }
  static get styles() {
    return [sharedStyles];
  }
}
`
});
    