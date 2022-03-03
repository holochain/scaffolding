import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const contextsTs = (): ScFile => ({
  type: ScNodeType.File,
  content: `import { Context, createContext } from '@holochain-open-dev/context';
import { AppWebsocket, InstalledAppInfo } from '@holochain/client';

export const appWebsocketContext: Context<AppWebsocket> = createContext(
  'appWebsocket'
);
export const appInfoContext: Context<InstalledAppInfo> = createContext('appInfo');
`
});
    