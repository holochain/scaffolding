import { FileChanges, FileChangesType } from '../../types';

import defaultNix from './default.nix';

export function generateNixFile(): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'default.nix',
      content: defaultNix({
        holochainVersion: {
          rev: 'holochain-0.0.108',
          sha256: '1p9rqd2d2wlyzc214ia93b1f18fgqspmza863q4hrz9ba6xigzjs',
          cargoSha256: '0p4m8ckbd7v411wgh14p0iz4dwi84i3cha5m1zgnqlln0wkqsb0f',
        },
        lairKeystoreHashes: {
          sha256: '0khg5w5fgdp1sg22vqyzsb2ri7znbxiwl7vr2zx6bwn744wy2cyv',
          cargoSha256: '1lm8vrxh7fw7gcir9lq85frfd0rdcca9p7883nikjfbn21ac4sn4',
        },
      }),
    },
  ];
}
