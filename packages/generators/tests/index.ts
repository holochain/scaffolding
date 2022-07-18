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

// generate zomes
const integrityZomesDna1 = [
  {
    name: 'hihi_integrity', // caution: the _integrity part at the end is required here
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
    name: 'hihi2_integrity',
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
];

const integrityZomesDna2 = [
  {
    name: 'hihi_integrity',
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
];

test('generate a full blown happ', async t => {
  const happChanges = webHapp(
    {
      name: 'haha',
      dnas: [
        {
          name: 'hehe',
          integrity_zomes: integrityZomesDna1,
          coordinator_zomes: [
            {
              name: 'hihi',
              dependencies: ['hihi_integrity'],
            },
            {
              name: 'hihi2',
              dependencies: ['hihi2_integrity'],
            },
          ],
        },
        {
          name: 'hehe2',
          integrity_zomes: integrityZomesDna2,
          coordinator_zomes: [
            {
              name: 'hihi',
              dependencies: ['hihi_integrity'],
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
