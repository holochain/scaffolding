import '@lit-labs/ssr/lib/render-with-global-dom-shim.js';

import test from 'tape';
import path from 'path';
import { writeDirectoryTree } from '@source-craft/fs';
import { webHapp } from '../dist';
import { holochainEntryTypeDefinition } from '@holochain-scaffolding/definitions';

import { fileURLToPath } from 'url';
import { ScNodeType } from '@source-craft/types';

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

test('generate a full blown happ', async t => {
  const happChanges = webHapp(
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
                  create: true,
                  update: true,
                  delete: false,
                  read: true,
                  typeDefinition: holochainEntryTypeDefinition('sample_entry', [
                    {
                      name: 'title',
                      type: 'Title',
                      configuration: {},
                    },
                  ]),
                },
                {
                  create: true,
                  update: false,
                  delete: false,
                  read: true,
                  typeDefinition: holochainEntryTypeDefinition('sample_entry2', [
                    {
                      name: 'title',
                      type: 'Content',
                      configuration: {},
                    },
                  ]),
                },
              ],
            },
            {
              name: 'hihi2',
              entry_defs: [
                {
                  create: true,
                  update: false,
                  delete: true,
                  read: false,
                  typeDefinition: holochainEntryTypeDefinition('sample_entry3', [
                    {
                      name: 'title',
                      type: 'DateTime',
                      configuration: {},
                    },
                  ]),
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
                  create: true,
                  update: false,
                  delete: false,
                  read: true,
                  typeDefinition: holochainEntryTypeDefinition('sample_entry', [
                    {
                      name: 'title',
                      type: 'EntryHash',
                      configuration: {},
                    },
                  ]),
                },
              ],
            },
          ],
        },
      ],
    },
    {
      type: ScNodeType.Directory,
      children: {},
    },
  );

  writeDirectoryTree(__dirname + '/.fixture', happChanges);

  t.equal(1, 1);
  t.end();
});
