import { FileChanges, FileChangesType } from '../../types';

import defaultNix from './default.nix';

export function generateNixFile(): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'default.nix',
      content: defaultNix({
        holochainVersion: {
          rev: 'holochain-0.0.109',
          sha256: '1rwss1y8cd52ccd0875pfpbw6v518vcry3hjc1lja69x2g2x12qb',
          cargoSha256: '08a72d7nqpakml657z9vla739cbg8y046av4pwisdgj1ykyzyi60',
        },
        lairKeystoreHashes: {
          sha256: '12n1h94b1r410lbdg4waj5jsx3rafscnw5qnhz3ky98lkdc1mnl3',
          cargoSha256: '0axr1b2hc0hhik0vrs6sm412cfndk358grfnax9wv4vdpm8bq33m',
        },
      }),
    },
  ];
}
