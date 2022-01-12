import { FileChanges, FileChangesType } from '../file-changes';

import defaultNix from './default.nix';

export function generateNixFile(): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'default.nix',
      content: defaultNix('d15633710a8d4349dc0ff03b7b47ad01eb9f2433'),
    },
  ];
}
