import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const agentAvatarTs = ({moduleNameSnakeCase, moduleNamePluralTitleCase, kebabPlural_, moduleNameTitleCase, moduleNamePlural, moduleName}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; kebabPlural_: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `import { AgentPubKeyB64 } from '@holochain-open-dev/core-types';
import { contextProvided } from '@holochain-open-dev/context';
import { ScopedElementsMixin } from '@open-wc/scoped-elements';
import { html, LitElement } from 'lit';
import { property } from 'lit/decorators.js';
import { ${camelCase(moduleNamePlural)}StoreContext } from '../context';
import { ${moduleNamePluralTitleCase}Store } from '../${kebabPlural_}store';
import { HoloIdenticon } from './holo-identicon';
import { SlAvatar, SlSkeleton } from '@scoped-elements/shoelace';
import { StoreSubscriber } from 'lit-svelte-stores';
import { sharedStyles } from './utils/shared-styles';

export class AgentAvatar extends ScopedElementsMixin(LitElement) {
  /** Public properties */

  /**
   * REQUIRED. The public key identifying the agent whose ${camelCase(moduleName)} is going to be shown.
   */
  @property({
    attribute: 'agent-pub-key',
    type: String,
  })
  agentPubKey!: AgentPubKeyB64;

  /**
   * Size of the avatar image in pixels.
   */
  @property({ type: Number })
  size = 32;

  /** Dependencies */

  /**
   * \`${moduleNamePluralTitleCase}Store\` that is requested via context.
   * Only set this property if you want to override the store requested via context.
   */
  @contextProvided({ context: ${camelCase(moduleNamePlural)}StoreContext })
  @property({ type: Object })
  store!: ${moduleNamePluralTitleCase}Store;

  private ${moduleNameSnakeCase} = new StoreSubscriber(this, () =>
    this.store?.${camelCase(moduleName)}Of(this.agentPubKey)
  );

  async firstUpdated() {
    if (this.store.config.avatarMode === 'avatar') {
      await this.store.fetchAgent${moduleNameTitleCase}(this.agentPubKey);
    }
  }

  render() {
    if (this.store.config.avatarMode === 'identicon')
      return html\`<holo-identicon
        .hash=\${this.agentPubKey}
        .size=\${this.size}
      ></holo-identicon>\`;
    if (this.${moduleNameSnakeCase}.value)
      return html\`
        <sl-avatar
          .image=\${this.${moduleNameSnakeCase}.value.fields.avatar}
          style="--size: \${this.size}px;"
        >
          <div slot="icon"></div>
        </sl-avatar>
      \`;
    return html\`<sl-skeleton
      effect="pulse"
      style="height: \${this.size}px; width: \${this.size}px"
    ></sl-skeleton>\`;
  }

  /**
   * @ignore
   */
  static get scopedElements() {
    return {
      'holo-identicon': HoloIdenticon,
      'sl-avatar': SlAvatar,
      'sl-skeleton': SlSkeleton,
    };
  }

  static styles = [sharedStyles];
}
`
});
    