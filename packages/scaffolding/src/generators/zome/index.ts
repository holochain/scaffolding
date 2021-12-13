import { FileChanges, FileChangesType } from '../../types/file-changes';
import { ZomeDefinition } from '../../types/zome';

import cargoToml from './Cargo.toml';
import libRs from './lib.rs';

export function generateZomeCargoToml(zomeName: string, author: string, hdkVersion = '0.0.115'): FileChanges[] {
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

export function generateZomeCode(zomeName: string): FileChanges[] {
  return [
    {
      type: FileChangesType.InDir,
      dirName: 'src',
      changes: [
        {
          type: FileChangesType.Create,
          fileName: `lib.rs`,
          content: libRs(),
        },
      ],
    },
  ];
}

export function generateZome(zome: ZomeDefinition): FileChanges[] {
  return [...generateZomeCargoToml(zome.name, '<AUTHOR>'), ...generateZomeCode(zome.name)];
}
