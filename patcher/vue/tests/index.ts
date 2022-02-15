import test from 'tape';
import { provideServiceForApp, patchEnvVars, generateVueApp, vueComponent } from '../dist';
import path from 'path';
import { PatcherDirectory, PatcherFile } from '@patcher/types';
import { applyPatch } from '@patcher/fs';

import { fileURLToPath } from 'url';
// @ts-ignore
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test('create a vue component', async t => {
  let vueApp = generateVueApp();

  vueApp = patchEnvVars(vueApp, { start: { VITE_HC_PORT: '$HC_PORT' } });

  vueApp = provideServiceForApp(vueApp, {
    service: {
      name: 'appWs',
      type: 'number',
    },
    createFnContent: 'return 3;',
    imports: [],
  });

  applyPatch(`${__dirname}/.fixture`, vueApp);
});
