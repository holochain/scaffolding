import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const myKebabTs = ({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural}: {moduleNamePluralTitleCase: string; _kebab: string; kebabPlural_: string; kebabSingular_: string; moduleNameTitleCase: string; moduleNamePlural: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `import { contextProvided } from '@holochain-open-dev/context';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { html, LitElement } from 'lit';
import { property, state } from 'lit/decorators.js';

import { ${camelCase(moduleNamePlural)}StoreContext } from '../context';
import { ${moduleNamePluralTitleCase}Store } from '../${kebabPlural_}store';
import { sharedStyles } from './utils/shared-styles';
import { Edit${moduleNameTitleCase} } from './edit${_kebab}';
import { ${moduleNameTitleCase}Detail } from './${kebabSingular_}detail';
import { IconButton } from '@scoped-elements/material-web';
import { Update${moduleNameTitleCase} } from './update${_kebab}';

/**
 * @element ${kebabSingular_}detail
 */
export class My${moduleNameTitleCase} extends ScopedElementsMixin(LitElement) {
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
  private _editing = false;

  render() {
    if (this._editing)
      return html\`<update${_kebab}
        @${kebabSingular_}updated=\${() => (this._editing = false)}
      ></update${_kebab}>\`;

    return html\`
      <${kebabSingular_}detail .agentPubKey=\${this.store.myAgentPubKey}>
        <mwc-icon-button
          slot="action"
          icon="edit"
          @click=\${() => (this._editing = true)}
        ></mwc-icon-button>
      </${kebabSingular_}detail>
    \`;
  }

  /**
   * @ignore
   */
  static get scopedElements() {
    return {
      'mwc-icon-button': IconButton,
      '${kebabSingular_}detail': ${moduleNameTitleCase}Detail,
      'update${_kebab}': Update${moduleNameTitleCase},
    };
  }

  static styles = [sharedStyles];
}
`
});
    