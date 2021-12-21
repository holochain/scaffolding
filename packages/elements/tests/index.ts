import test from 'tape';
import { generateWebHapp } from '../dist';
import path from 'path';
import { applyGeneratedChanges } from './utils';

import { fileURLToPath } from 'url';
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test('scaffold new application', t => {
  const happChanges = generateWebHapp({
    name: 'haha',
    dnas: [
      {
        name: 'hehe',
        zomes: [
          {
            name: 'hihi',
          },
          {
            name: 'hihi2',
          },
        ],
      },
      {
        name: 'hehe2',
        zomes: [
          {
            name: 'hihi3',
          },
          {
            name: 'hihi4',
          },
        ],
      },
    ],
  });

  applyGeneratedChanges(__dirname + '/.fixture', happChanges);

  t.equal(1, 1);
  t.end();
});
