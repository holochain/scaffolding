import test from 'tape';
import { readFolder, applyPatch, directoryToPatcher } from '../dist';
import path from 'path';
import { PatcherDirectory, PatcherFile } from '@patcher/types';

import { fileURLToPath } from 'url';
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test('read the PatcherDirectory for this package', async t => {
  const d = readFolder(`${__dirname}/../`);
  t.equal(5, Object.keys(d.children).length);

  applyPatch(`${__dirname}/.fixture`, d);

  ((d.children['src'] as PatcherDirectory).children['index.ts'] as PatcherFile).content += 'hello';

  applyPatch(`${__dirname}/.fixture`, d);
});

test('generate a patcher for this package', async t => {
  const d = readFolder(`${__dirname}/../`);
  
  const patcher = directoryToPatcher(d, {
    'Dictionary': 'Pictionary'
  });

  applyPatch(`${__dirname}/.patcher`, patcher)
});
