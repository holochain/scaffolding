import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const holochainAppTs = ({happName, subcomponentImports, appContent}: {happName: string; subcomponentImports: string; appContent: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `import '@webcomponents/scoped-custom-element-registry';

import { LitElement, css, html } from 'lit';
import { customElement, property, state } from 'lit/decorators.js';
import {
  AppWebsocket,
  EntryHash,
  InstalledAppInfo,
} from '@holochain/client';
import { contextProvider } from '@lit-labs/context';
import '@material/mwc-circular-progress';

${subcomponentImports}
import { appWebsocketContext, appInfoContext } from './contexts';

@customElement('holochain-app')
export class HolochainApp extends LitElement {
  @state() loading = true;
  @state() entryHash: EntryHash | undefined;

  @contextProvider({ context: appWebsocketContext })
  @property({ type: Object })
  appWebsocket!: AppWebsocket;

  @contextProvider({ context: appInfoContext })
  @property({ type: Object })
  appInfo!: InstalledAppInfo;

  async firstUpdated() {
    this.appWebsocket = await AppWebsocket.connect(
      \`ws://localhost:\${process.env.HC_PORT}\`
    );

    this.appInfo = await this.appWebsocket.appInfo({
      installed_app_id: '${happName}',
    });

    this.loading = false;
  }

  render() {
    if (this.loading)
      return html\`
        <mwc-circular-progress indeterminate></mwc-circular-progress>
      \`;

    return html\`
      <main>
        <h1>${happName}</h1>

        ${appContent}
      </main>
    \`;
  }

  static styles = css\`
    :host {
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      justify-content: flex-start;
      font-size: calc(10px + 2vmin);
      color: #1a2b42;
      max-width: 960px;
      margin: 0 auto;
      text-align: center;
      background-color: var(--lit-element-background-color);
    }

    main {
      flex-grow: 1;
    }

    .app-footer {
      font-size: calc(12px + 0.5vmin);
      align-items: center;
    }

    .app-footer a {
      margin-left: 5px;
    }
  \`;
}
`
});
    