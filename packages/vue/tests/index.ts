import '@lit-labs/ssr/lib/render-with-global-dom-shim.js';
import test from 'tape';
import path from 'path';
import { generateVueApp } from '../dist';
import { writeDirectoryTree } from '@source-craft/fs';

import { fileURLToPath } from 'url';
import { newHappDef } from '@holochain-scaffolding/definitions';
import { webHapp } from '@holochain-scaffolding/generators';

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test('create a vue component', async t => {
  const happDef = newHappDef('hello-world');
  const vueApp = generateVueApp(happDef);

  const generatedWebHapp = webHapp(happDef, vueApp);

  writeDirectoryTree(`${__dirname}/.fixture`, generatedWebHapp);
});
