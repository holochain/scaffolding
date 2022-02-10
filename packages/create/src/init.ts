import { WebFramework, webHapp } from '@holochain/rad-patcher';
import { applyPatch } from '@patcher/fs';

export async function init(appName: string) {
  const d = await webHapp(
    {
      name: appName,
      dnas: [
        {
          name: 'dna-1',
          zomes: [
            {
              entry_defs: [
                {
                  create: true,
                  delete: true,
                  update: true,
                  read: true,
                  sample: {
                    foo: 1,
                    bar: 'some content',
                  },
                  name: 'entry-def-1',
                },
              ],
              name: 'zome-1',
            },
          ],
        },
      ],
    },
    WebFramework.Vue,
  );
  applyPatch(`${process.cwd()}/${appName}`, d);
}
