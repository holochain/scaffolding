export interface HolochainVersion {
  rev: string;
  sha256: string;
  cargoSha256: string;
}
export interface LairKeystoreHashes {
  sha256: string;
  cargoSha256: string;
}

export default ({
  holochainVersion,
  lairKeystoreHashes,
}: {
  holochainVersion: HolochainVersion;
  lairKeystoreHashes: LairKeystoreHashes;
}) =>
  `{
  holonixPath ?  builtins.fetchTarball { url = "https://github.com/holochain/holonix/archive/a0dcdfac2c8783c58805175dd5bc5528ccbb35fd.tar.gz"; }
}:

let
  holonix = import (holonixPath) {
    include = {
        # making this explicit even though it's the default
        holochainBinaries = true;
    };

    holochainVersionId = "custom";

    holochainVersion = {
      rev = "${holochainVersion.rev}";
      sha256 = "${holochainVersion.sha256}";
      cargoSha256 = "${holochainVersion.cargoSha256}";
      bins = {
        holochain = "holochain";
        hc = "hc";
      };

      lairKeystoreHashes = {
        sha256 = "${lairKeystoreHashes.sha256}";
        cargoSha256 = "${lairKeystoreHashes.cargoSha256}";
      };
    };
  };
  nixpkgs = holonix.pkgs;
in nixpkgs.mkShell {
  inputsFrom = [ holonix.main ];
  buildInputs = with nixpkgs; [
    binaryen
    nodejs-16_x
  ];
}
`;
