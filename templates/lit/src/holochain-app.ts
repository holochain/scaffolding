import { LitElement, css, html } from 'lit';
import { customElement, property } from 'lit/decorators.js';
import { AppWebsocket } from '@holochain/conductor-api';

@customElement('holochain-app')
export class HolochainApp extends LitElement {
  @property() title = 'My app';

  async firstUpdated() {
    const appWebsocket = await AppWebsocket.connect(
      `ws://localhost:${process.env.HC_PORT}`
    );

    const appInfo = await appWebsocket.appInfo({
      installed_app_id: 'HC_SCAFFOLD{installedAppId}',
    });

    const cellData = appInfo.cell_data[0];

    const postHash = await appWebsocket.callZome({
      cap: null as any,
      cell_id: cellData.cell_id,
      zome_name: 'HC_SCAFFOLD{zomeName}',
      fn_name: 'create_post',
      payload: 'my post',
      provenance: cellData.cell_id[1],
    });
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
