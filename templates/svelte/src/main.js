import App from './App.svelte';
import { AppWebsocket } from '@holochain/conductor-api';

async function setup() {
  const appWebsocket = await AppWebsocket.connect(`ws://localhost:${process.env.HC_PORT}`);

  const app = new App({
    target: document.body,
    props: {
      name: 'world',
      appWebsocket,
    },
  });
}

export default setup();
