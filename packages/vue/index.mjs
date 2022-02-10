import { readFolder, directoryToPatcher, applyPatch } from '@patcher/fs';
import { fileURLToPath } from 'url';
import path from 'path';
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const d = readFolder(`${__dirname}/template`);

const patched = directoryToPatcher(d, {
  'zome-name': 'zomeName',
  'fn-name': 'fnName',
  "{ my: 'sample-payload' }": 'payload',
  'my-cell-role': 'cellRole',
  'test-app': 'installedAppId'
});

applyPatch(`${__dirname}/src`, patched);
