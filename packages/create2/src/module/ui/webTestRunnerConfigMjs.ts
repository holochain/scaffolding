import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const webTestRunnerConfigMjs = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `import plugins from './web-dev.plugins.mjs';
import { puppeteerLauncher } from '@web/test-runner-puppeteer';

const files = ['test/**/*.test.js'];
// If the CONDUCTOR_URL property is set, we are doing E2E tests with holochain up and ready
if (process.env.CONDUCTOR_URL) files.push('e2e/**/*.test.js');

const debug = !!process.env.DEBUG;

export default {
  files,
  manual: debug,
  open: debug,
  nodeResolve: {
    browser: true,
  },
  plugins,
  browsers: [
    puppeteerLauncher({
      launchOptions: {
        args: ['--no-sandbox'],
      },
    }),
  ],
};
`
});
    