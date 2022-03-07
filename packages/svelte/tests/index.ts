import test from 'tape';
import path from 'path';
import { generateSvelteApp } from '../dist';
import { writeDirectoryTree } from '@source-craft/fs';

import { fileURLToPath } from 'url';
import { newHappDef } from '@holochain-scaffolding/definitions';
import { webHapp } from '@holochain-scaffolding/generators';

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test('generate a svelte webHapp', async t => {
  const happDef = newHappDef('hello-world');
  const svelteApp = generateSvelteApp(happDef);

  const generatedWebHapp = webHapp(happDef, svelteApp);

  writeDirectoryTree(`${__dirname}/.fixture`, generatedWebHapp);
});
