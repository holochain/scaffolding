import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const webDevServerConfigMjs = (): ScFile => ({
  type: ScNodeType.File,
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
    