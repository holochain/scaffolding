import { FileChanges, FileChangesType } from '../file-changes';

import defaultNix from './default.nix';

export function generateNixFile(): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'default.nix',
      content: defaultNix('1cb431ac2d30d6f44dbcb5a40520f7328ae49ec1'),
    },
  ];
}
