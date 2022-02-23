import { holochainEntryTypeDefinition } from '@holochain-scaffolding/definitions';
import { WebFramework, webHapp } from '@holochain-scaffolding/source-craft';
import { applyPatch } from '@source-craft/fs';

export function init(appName: string): void {
  const d = webHapp(
    {
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
    },
    WebFramework.Vue,
  );
  applyPatch(`${process.cwd()}/${appName}`, d);
}
