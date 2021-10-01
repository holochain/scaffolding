import { AppWebsocket } from '@holochain/conductor-api';

let _appWebsocket: AppWebsocket | undefined;

export async function appWebsocket() {
  if (_appWebsocket) return _appWebsocket;

  _appWebsocket = await AppWebsocket.connect(`ws://localhost:${process.env.VUE_APP_HC_PORT}`);
  return _appWebsocket;
}
