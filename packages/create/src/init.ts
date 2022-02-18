import { holochainEntryTypeDefinition } from '@holochain-scaffolding/definitions';
import { WebFramework, webHapp } from '@holochain-scaffolding/patcher';
import { applyPatch } from '@patcher/fs';

export async function init(appName: string) {
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
