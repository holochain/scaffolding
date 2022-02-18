import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const shimDTs = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `declare module '@holo-host/identicon';
`
});
    