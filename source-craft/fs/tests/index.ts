import test from 'tape';
import { readFolder, applyPatch, directoryTo@holochain-scaffolding/source-craft } from '../dist';
import path from 'path';
import { ScDirectory, ScFile } from '@source-craft/types';

import { fileURLToPath } from 'url';
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test('read the ScDirectory for this package', async t => {
  const d = readFolder(`${__dirname}/../`);
  t.equal(5, Object.keys(d.children).length);

  applyPatch(`${__dirname}/.fixture`, d);

  ((d.children['src'] as ScDirectory).children['index.ts'] as ScFile).content += 'hello';

  applyPatch(`${__dirname}/.fixture`, d);
});

test('generate a @holochain-scaffolding/source-craft for this package', async t => {
  const d = readFolder(`${__dirname}/../`);

  const @holochain-scaffolding/source-craft = directoryToGenerator(d, [
    {
      literal: 'Dictionary',
      template: 'Pictionary',
    },
  ]);

  applyPatch(`${__dirname}/.@holochain-scaffolding/source-craft`, @holochain-scaffolding/source-craft);
});
