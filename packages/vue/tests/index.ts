import '@lit-labs/ssr/lib/render-with-global-dom-shim.js';
import test from 'tape';
import path from 'path';
import { generateVueApp } from '../dist';
import { writeDirectoryTree } from '@source-craft/fs';

import { fileURLToPath } from 'url';
import { HappDefinition, holochainEntryTypeDefinition, newHappDef } from '@holochain-scaffolding/definitions';
import { webHapp } from '@holochain-scaffolding/generators';

// eslint-disable-next-line @typescript-eslint/ban-ts-comment
// @ts-ignore
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const integrityZomesDna1 = [
  {
    name: 'zome_1_integrity',
    entry_defs: [
      {
        create: true,
        delete: true,
        update: true,
        read: true,
        typeDefinition: holochainEntryTypeDefinition('entry-def-1', [
          {
            name: 'title',
            type: 'Title',
            configuration: {},
          },
          {
            name: 'content',
            type: 'Content',
            configuration: {},
          },
        ]),
      },
      {
        create: true,
        delete: true,
        update: true,
        read: true,
        typeDefinition: holochainEntryTypeDefinition('entry-def-2', [
          {
            name: 'author',
            type: 'AgentPubKey',
            configuration: {},
          },
          {
            name: 'dna',
            type: 'ActionHash',
            configuration: {},
          },
        ]),
      },
    ],
  },
  {
    name: 'zome_2_integrity',
    entry_defs: [
      {
        create: true,
        delete: true,
        update: true,
        read: true,
        typeDefinition: holochainEntryTypeDefinition('entry-def-1', [
          {
            name: 'title',
            type: 'Title',
            configuration: {},
          },
          {
            name: 'content',
            type: 'Content',
            configuration: {},
          },
        ]),
      },
      {
        create: true,
        delete: true,
        update: true,
        read: true,
        typeDefinition: holochainEntryTypeDefinition('entry-def-2', [
          {
            name: 'author',
            type: 'AgentPubKey',
            configuration: {},
          },
          {
            name: 'dna',
            type: 'ActionHash',
            configuration: {},
          },
        ]),
      },
    ],
  },
];

test('create a vue component', async t => {
  const happDef: HappDefinition = {
    name: 'hello-world',
    dnas: [
      {
        name: 'dna_1',
        integrity_zomes: integrityZomesDna1,
        coordinator_zomes: integrityZomesDna1.map(iz => ({
          name: iz.name.slice(0, iz.name.length - 10),
          dependencies: [iz.name],
        })),
      },
    ],
  };
  const vueApp = generateVueApp(happDef);

  const generatedWebHapp = webHapp(happDef, vueApp);

  writeDirectoryTree(`${__dirname}/.fixture`, generatedWebHapp);
});
