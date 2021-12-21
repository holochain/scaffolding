import { FileChanges, FileChangesType } from '../file-changes';

import defaultNix from './default.nix';

export function generateNixFile(): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'default.nix',
      content: defaultNix('2f8ca2fa76165e2978112cb693c572f1086c5541'),
    },
  ];
}
