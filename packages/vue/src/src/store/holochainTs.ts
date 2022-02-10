export const holochainTs = () => `import { AppWebsocket, InstalledAppInfo } from '@holochain/client';

let _appWebsocket: AppWebsocket | undefined;

export async function appWebsocket() {
  if (_appWebsocket) return _appWebsocket;

  _appWebsocket = await AppWebsocket.connect(\`ws://localhost:\${process.env.VUE_APP_HC_PORT}\`);
  return _appWebsocket;
}

let _appInfo: InstalledAppInfo | undefined;

export async function appInfo() {
  if (_appInfo) return _appInfo;
  const appWs = await appWebsocket();

  _appInfo = await appWs.appInfo({
    installed_app_id: 'test-app',
  });

  return _appInfo;
}
`
    