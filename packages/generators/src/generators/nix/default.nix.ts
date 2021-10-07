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
  holonixPath ?  builtins.fetchTarball { url = "https://github.com/holochain/holonix/archive/48a75e79b1713334ab0086767a214e5b1619d38d.tar.gz"; }
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
