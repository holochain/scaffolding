import '@lit-labs/ssr/lib/render-with-global-dom-shim.js';
import test from 'tape';
import path from 'path';
import { generateVueApp } from '../dist';
import { writeDirectoryTree } from '@source-craft/fs';

import { fileURLToPath } from 'url';
import { HappDefinition, holochainEntryTypeDefinition, newHappDef } from '@holochain-scaffolding/definitions';
import { webHapp, zomeBundlesForIntegrityZomes } from '@holochain-scaffolding/generators';

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
]

const zomeBundlesDna1 = zomeBundlesForIntegrityZomes(integrityZomesDna1);

test('create a vue component', async t => {
  const happDef: HappDefinition = {
    name: 'hello-world',
    dnas: [
      {
        name: 'dna_1',
        zomeBundles: zomeBundlesDna1,
      },
    ],
  };
  const vueApp = generateVueApp(happDef);

  const generatedWebHapp = webHapp(happDef, vueApp);

  writeDirectoryTree(`${__dirname}/.fixture`, generatedWebHapp);
});
