import App from './App.svelte';
import { AppWebsocket } from '@holochain/conductor-api';

async function setup() {
  const appWebsocket = await AppWebsocket.connect(`ws://localhost:${process.env.HC_PORT}`);

  const appInfo = await appWebsocket.appInfo({
    installed_app_id: 'HC_SCAFFOLD{installedAppId}',
  });

  const cellData = appInfo.cell_data[0];

  const app = new App({
    target: document.body,
    props: {
      name: 'world',
      appWebsocket,
      cell_id: cellData.cell_id
    },
  });
}

export default setup();
