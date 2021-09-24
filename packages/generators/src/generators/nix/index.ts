import { FileChanges, FileChangesType } from '../../types';

import defaultNix from './default.nix';

export function generateNixFile(): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'default.nix',
      content: defaultNix({
        holochainVersion: {
          rev: 'a1206a694fe3b521440fe633db99a50b8255c1b2',
          sha256: '0qdjjkqw3xlg8g686gvn509a9rv4kc6qfw07hypzc0fksix9d4iz',
          cargoSha256: '175b76j31sls0gj08imchwnk7n4ylsxlc1bm58zrhfmq62hcchb1',
        },
        lairKeystoreHashes: {
          sha256: '0khg5w5fgdp1sg22vqyzsb2ri7znbxiwl7vr2zx6bwn744wy2cyv',
          cargoSha256: '1lm8vrxh7fw7gcir9lq85frfd0rdcca9p7883nikjfbn21ac4sn4',
        },
      }),
    },
  ];
}
