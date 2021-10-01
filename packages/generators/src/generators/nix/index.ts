import { FileChanges, FileChangesType } from '../../types';

import defaultNix from './default.nix';

export function generateNixFile(): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'default.nix',
      content: defaultNix({
        holochainVersion: {
          rev: '4347e4dbfd4f957576275fb2e8b3deda90ccbfd7',
          sha256: 'sha256:1g3sm1x786zr9w324kxlsf50ajrmpigjj6l1xnm1cwl2hbqq7hxz',
          cargoSha256: 'sha256:1i6i80vf7jjw1h0b3dsh5n0x8g5g3h16sw9rskw84yipqbv51nc7',
        },
        lairKeystoreHashes: {
          sha256: '1ibynj1mn1mc59x7b2jn8l1vv9m8czwcvpq81qgbpa52jgjqlf14',
          cargoSha256: '1dnfjdk3b4l7ysvm81r061mxly889bbcmg2h11nkgmfj79djka9s',
        },
      }),
    },
  ];
}
