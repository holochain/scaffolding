import { LitElement, css, html } from 'lit';
import { customElement, state } from 'lit/decorators.js';
import { AppWebsocket, InstalledCell } from '@holochain/conductor-api';

@customElement('holochain-app')
export class HolochainApp extends LitElement {
  @state() entryHash: string | undefined;

  async firstUpdated() {
    const appWebsocket = await AppWebsocket.connect(
      `ws://localhost:${process.env.HC_PORT}`
    );

    const appInfo = await appWebsocket.appInfo({
      installed_app_id: 'HC_SCAFFOLDING{installedAppId}',
    });

    const cellData = appInfo.cell_data.find(data => data.role_id === 'HC_SCAFFOLDING{dnaName}') as InstalledCell;

    const result = await appWebsocket.callZome({
      cap: null as any,
      cell_id: cellData.cell_id,
      zome_name: 'HC_SCAFFOLDING{zomeName}',
      fn_name: 'HC_SCAFFOLDING{fnName}',
      payload: HC_SCAFFOLDING{entrySample},
      provenance: cellData.cell_id[1],
    });
    this.entryHash = result.entry_hash
  }

  render() {
    return html`
      <main>
        <h1>${this.title}</h1>

        <p>Edit <code>src/holochain-app.ts</code> and save to reload.</p>
        <a
          class="app-link"
          href="https://open-wc.org/guides/developing-components/code-examples"
          target="_blank"
          rel="noopener noreferrer"
        >
          Code examples
        </a>
      </main>

      ${this.entryHash
        ? html`<span
            >Created new Holochain entry! HC_SCAFFOLDING{entryDefName} with hash: ${this.entryHash}</span
          >`
        : html`<span>Creating...</span>`}

      <p class="app-footer">
        🚽 Made with love by
        <a
          target="_blank"
          rel="noopener noreferrer"
          href="https://github.com/open-wc"
          >open-wc</a
        >.
      </p>
    `;
  }

  static styles = css`
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
  `;
}
