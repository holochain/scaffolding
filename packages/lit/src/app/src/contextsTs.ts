import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const contextsTs = (): ScFile => ({
  type: ScNodeType.File,
  content: `import { createContext } from '@lit-labs/context';
import { AppWebsocket, InstalledAppInfo } from '@holochain/client';

export const appWebsocketContext = createContext<AppWebsocket>('appWebsocket');
export const appInfoContext = createContext<InstalledAppInfo>('appInfo');
`
});
    