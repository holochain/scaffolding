import test from 'tape';
import { generateWebHapp } from '../dist';
import path from 'path';
import { applyGeneratedChanges } from './utils';

import { fileURLToPath } from 'url';
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test('generate a full blown happ', async t => {
  const happChanges = await generateWebHapp(
    {
      name: 'haha',
      dnas: [
        {
          name: 'hehe',
          zomes: [
            {
              name: 'hihi',
              entry_defs: [
                {
                  name: 'sample_entry',
                  create: true,
                  update: true,
                  delete: false,
                  read: true,
                  sample: { foo: 'hi', bar: 3 },
                },
                {
                  name: 'sample_entry2',
                  create: true,
                  update: false,
                  delete: false,
                  read: true,
                  sample: { foo: 'hi', bar: 3 },
                },
              ],
            },
            {
              name: 'hihi2',
              entry_defs: [
                {
                  name: 'sample_entry',
                  create: true,
                  update: false,
                  delete: true,
                  read: false,
                  sample: { foo: 'hi', bar: 3 },
                },
              ],
            },
          ],
        },
        {
          name: 'hehe2',
          zomes: [
            {
              name: 'hihi',
              entry_defs: [
                {
                  name: 'sample_entry',
                  create: true,
                  update: false,
                  delete: false,
                  read: true,
                  sample: { foo: 'hi', bar: 3 },
                },
              ],
            },
          ],
        },
      ],
    },
    [],
  );

  applyGeneratedChanges(__dirname + '/.fixture', happChanges);

  t.equal(1, 1);
  t.end();
});
