import App from './App.svelte';
import { AppWebsocket, InstalledCell } from '@holochain/conductor-api';

async function setup() {
  const appWebsocket = await AppWebsocket.connect(`ws://localhost:${process.env.HC_PORT}`);

  const appInfo = await appWebsocket.appInfo({
    installed_app_id: 'HC_SCAFFOLDING{installedAppId}',
  });

  const cellData = appInfo.cell_data.find(data => data.role_id === 'HC_SCAFFOLDING{dnaName}') as InstalledCell;

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
