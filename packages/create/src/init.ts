import { HappDefinition, holochainEntryTypeDefinition } from '@holochain-scaffolding/definitions';
import { generateVueApp } from '@holochain-scaffolding/vue';
import { webHapp } from '@holochain-scaffolding/generators';
import { writeDirectoryTree } from '@source-craft/fs';
import { automaticSetup } from './events/automatic-setup';

export async function init(appName: string): Promise<void> {
  const happDef: HappDefinition = {
    name: appName,
    dnas: [
      {
        name: 'dna_1',
        zomes: [
          {
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
            name: 'zome_1',
          },
        ],
      },
    ],
  };
  const d = generateVueApp(happDef);

  const app = webHapp(happDef, d);
  writeDirectoryTree(`${process.cwd()}/${appName}`, app);

  await automaticSetup(appName);
}
