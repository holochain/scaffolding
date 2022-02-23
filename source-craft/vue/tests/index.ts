import test from 'tape';
import { provideContextForApp, patchEnvVars, generateVueApp } from '../dist';
import path from 'path';
import { ScDirectory, ScFile } from '@source-craft/types';
import { applyPatch } from '@source-craft/fs';

import { fileURLToPath } from 'url';
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test('create a vue component', async t => {
  let vueApp = generateVueApp();

  vueApp = patchEnvVars(vueApp, { start: { VITE_HC_PORT: '$HC_PORT' } });

  vueApp = provideContextForApp(vueApp, {
    context: {
      imports: [],
      name: 'appWs',
      type: 'number',
    },
    createContext: {
      async: false,
      fnContent: 'return 3;',
      imports: [],
      params: [],
    },
  });

  applyPatch(`${__dirname}/.fixture`, vueApp);
});
