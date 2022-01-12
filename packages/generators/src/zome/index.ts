import { ZomeDefinition } from '@holochain/rad-definitions';

import { FileChanges, FileChangesType } from '../file-changes';

import cargoToml from './Cargo.toml';
import { generateEntryDef } from './entry';
import libRs from './lib.rs';

export * from './entry';

export function generateZomeCargoToml(zomeName: string, author: string, hdkVersion = '0.0.118'): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: `Cargo.toml`,
      content: cargoToml({
        zomeName: zomeName,
        author,
        hdkVersion,
      }),
    },
  ];
}

export async function generateZomeCode(zomeDefinition: ZomeDefinition): Promise<FileChanges[]> {
  const promises = zomeDefinition.entry_defs.map(
    async entryDef =>
      ({
        type: FileChangesType.InDir,
        dirName: entryDef.name,
        changes: await generateEntryDef(entryDef),
      } as FileChanges),
  );

  const entryDefs: FileChanges[] = await Promise.all(promises);
  return [
    {
      type: FileChangesType.InDir,
      dirName: 'src',
      changes: [
        {
          type: FileChangesType.Create,
          fileName: `lib.rs`,
          content: libRs(zomeDefinition),
        },
        ...entryDefs,
      ],
    },
  ];
}

export async function generateZome(zome: ZomeDefinition): Promise<FileChanges[]> {
  return [...generateZomeCargoToml(zome.name, '<AUTHOR>'), ...(await generateZomeCode(zome))];
}
