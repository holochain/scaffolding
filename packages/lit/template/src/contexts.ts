import { Context, createContext } from '@holochain-open-dev/context';
import { AppWebsocket, InstalledAppInfo } from '@holochain/client';

export const appWebsocketContext: Context<AppWebsocket> = createContext(
  'appWebsocket'
);
export const appInfoContext: Context<InstalledAppInfo> = createContext('appInfo');
