import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const webDevServerConfigMjs = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `import plugins from './web-dev.plugins.mjs';

export default {
  watch: true,
  nodeResolve: {
    browser: true,
    preferBuiltins: false,
    exportConditions: ['browser', 'development'],
  },
  appIndex: 'demo/index.html',
  rootDir: '../',
  open: true,
  plugins,
};
`
});
    