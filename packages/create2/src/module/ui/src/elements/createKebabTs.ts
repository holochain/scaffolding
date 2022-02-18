import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const createKebabTs = ({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName}: {moduleNamePluralTitleCase: string; _kebab: string; kebabPlural_: string; kebabSingular_: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
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
} from '@scoped-elements/material-web';
import { SlAvatar } from '@scoped-elements/shoelace';

import { sharedStyles } from './utils/shared-styles';
import { ${moduleNamePluralTitleCase}Store } from '../${kebabPlural_}store';
import { ${camelCase(moduleNamePlural)}StoreContext } from '../context';
import { resizeAndExport } from './utils/image';
import { Edit${moduleNameTitleCase} } from './edit${_kebab}';
import { ${moduleNameTitleCase} } from '../types';

/**
 * A custom element that fires event on value change.
 *
 * @element create${_kebab}
 * @fires ${kebabSingular_}created - Fired after the ${camelCase(moduleName)} has been created. Detail will have this shape: { ${camelCase(moduleName)}: { nickname, fields } }
 */
export class Create${moduleNameTitleCase} extends ScopedElementsMixin(LitElement) {
  /** Dependencies */

  /**
   * \`${moduleNamePluralTitleCase}Store\` that is requested via context.
   * Only set this property if you want to override the store requested via context.
   */
  @contextProvided({ context: ${camelCase(moduleNamePlural)}StoreContext })
  @property({ type: Object })
  store!: ${moduleNamePluralTitleCase}Store;

  /** Private properties */

  async create${moduleNameTitleCase}(${camelCase(moduleName)}: ${moduleNameTitleCase}) {
    await this.store.create${moduleNameTitleCase}(${camelCase(moduleName)});

    this.dispatchEvent(
      new CustomEvent('${kebabSingular_}created', {
        detail: {
          ${camelCase(moduleName)},
        },
        bubbles: true,
        composed: true,
      })
    );
  }

  render() {
    return html\`
      <mwc-card>
        <div class="column" style="margin: 16px;">
          <span
            class="title"
            style="margin-bottom: 24px; align-self: flex-start"
            >Create ${moduleNameTitleCase}</span
          >
          <edit${_kebab}
            save${_kebab}-label="Create ${moduleNameTitleCase}"
            @save${_kebab}=\${(e: CustomEvent) =>
              this.create${moduleNameTitleCase}(e.detail.${camelCase(moduleName)})}
          ></edit${_kebab}></div
      ></mwc-card>
    \`;
  }

  /**
   * @ignore
   */
  static get scopedElements() {
    return {
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
    