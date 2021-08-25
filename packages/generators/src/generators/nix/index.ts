import { FileChanges, FileChangesType } from '../../types';

//@ts-ignore
import defaultNix from './default.nix.hbs';

export function generateNixFile(): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'default.nix',
      content: defaultNix({
        holochainVersion: {
          rev: 'f3d17d993ad8d988402cc01d73a0095484efbabb',
          sha256: '1z0y1bl1j2cfv4cgr4k7y0pxnkbiv5c0xv89y8dqnr32vli3bld7',
          cargoSha256: '1rf8vg832qyymw0a4x247g0iikk6kswkllfrd5fqdr0qgf9prc31',
        },
        lairKeystoreHashes: {
          sha256: '1jiz9y1d4ybh33h1ly24s7knsqyqjagsn1gzqbj1ngl22y5v3aqh',
          cargoSha256: '0agykcl7ysikssfwkjgb3hfw6xl0slzy38prc4rnzvagm5wd1jjv',
        },
      }),
    },
  ];
}
