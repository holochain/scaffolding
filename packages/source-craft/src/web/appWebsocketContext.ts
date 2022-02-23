import { Context } from '@source-craft/web-apps';

export const appWebsocketContext: Context = {
  imports: [`import { AppWebsocket, InstalledAppInfo } from '@holochain/client';`],
  name: 'holochainContext',
  type: '{ appWs: AppWebsocket; appInfo: InstalledAppInfo; }',
};
