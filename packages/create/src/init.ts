import { HappDefinition, holochainEntryTypeDefinition } from '@holochain-scaffolding/definitions';
import { generateVueApp } from '@holochain-scaffolding/vue';
import { webHapp } from '@holochain-scaffolding/generators';
import { writeDirectoryTree } from '@source-craft/fs';
import { automaticSetup } from './events/automatic-setup';

import { zomeBundlesForIntegrityZomes } from '@holochain-scaffolding/generators';

const integrityZomesDna1 = [
  {
    name: 'zome_1_integrity', // CAUTION: the "_integrity" part is required for zomeBundlesForIntegrityZomes() to work correctly
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
    ],
  },
];

const zomeBundlesDna1 = zomeBundlesForIntegrityZomes(integrityZomesDna1);



export async function init(appName: string): Promise<void> {
  const happDef: HappDefinition = {
    name: appName,
    dnas: [
      {
        name: 'dna_1',
        zomeBundles: zomeBundlesDna1,
      },
    ],
  };
  const d = generateVueApp(happDef);

  const app = webHapp(happDef, d);
  writeDirectoryTree(`${process.cwd()}/${appName}`, app);

  await automaticSetup(appName);
}
