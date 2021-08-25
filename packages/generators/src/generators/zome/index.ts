import { FileChanges, FileChangesType } from '../../types';
import { camelToSnakeCase } from '../utils';

//@ts-ignore
import cargoToml from './Cargo.toml.hbs';
//@ts-ignore
import libRs from './lib.rs.hbs';

export function generateZomeCargoToml(zomeName: string, author: string, hdkVersion = '0.0.103'): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: `Cargo.toml`,
      content: cargoToml({
        zomeName: camelToSnakeCase(zomeName),
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
          content: libRs({}),
        },
      ],
    },
  ];
}

export function generateZome(zomeName: string): FileChanges[] {
  return [...generateZomeCargoToml(zomeName, 'test'), ...generateZomeCode(zomeName)];
}
