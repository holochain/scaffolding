import { holochainEntryTypeDefinition } from '@holochain-scaffolding/definitions';
import { generateVueWebHapp } from '@holochain-scaffolding/vue';
import { applyPatch } from '@source-craft/fs';

export function init(appName: string): void {
  const d = generateVueWebHapp({
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
                typeDefinition: holochainEntryTypeDefinition('entry-def-1', []),
              },
            ],
            name: 'zome_1',
          },
        ],
      },
    ],
  });
  applyPatch(`${process.cwd()}/${appName}`, d);
}
