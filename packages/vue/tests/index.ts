import test from 'tape';
import path from 'path';
import { generateVueWebHapp } from '../dist';
import { applyPatch } from '@source-craft/fs';
import { importDeclaration } from '@source-craft/web-apps';

import { fileURLToPath } from 'url';
import { newHappDef } from '@holochain-scaffolding/definitions';

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test('create a vue component', async t => {
  const vueApp = generateVueWebHapp(newHappDef('hello-world'));

  applyPatch(`${__dirname}/.fixture`, vueApp);
});
