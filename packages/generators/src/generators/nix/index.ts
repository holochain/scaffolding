import { FileChanges, FileChangesType } from '../../types';

import defaultNix from './default.nix';

export function generateNixFile(): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'default.nix',
      content: defaultNix({
        holochainVersion: {
          rev: 'holochain-0.0.107',
          sha256: '1m5clhh0xpr4ajdbybxjqc5vblkd30lsfb1sac4zbzxjrnpp5iki',
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
