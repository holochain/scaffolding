import { HappDefinition } from '@holochain/rad-definitions';
import { PatcherFile, PatcherNodeType } from '@patcher/types';

import { getDnaPath, getUiPackageName, mergeStrings } from '../utils';

export const rootPackageJson = (happ: HappDefinition): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `{
  "name": "${happ.name}-dev",
  "private": true,
  "workspaces": [
    "ui",
    "tests"
  ],
  "scripts": {
    "start": "npm run build:happ && npm run start:agent",
    "network": "npm run build:happ && concurrently-repeat \\"npm run start:agent\\"",
    "start:agent": "cross-env HC_PORT=$(port) concurrently -k \\"npm run start:happ\\" \\"sleep 5 && npm run start -w ${getUiPackageName(
      happ,
    )}\\"",
    "test": "npm run build:happ && npm t -w tests",
    "start:happ": "concurrently \\"RUST_LOG=warn hc s generate ./workdir/${happ.name}.happ --run=$HC_PORT -a ${
    happ.name
  } network mdns\\" \\"npm run playground\\"",
    "package": "npm run build:happ && npm run package:ui && hc web-app pack workdir",
    "package:ui": "npm run build -w ${getUiPackageName(happ)} && cd ui/dist && bestzip ../dist.zip *",
    "build:happ": "npm run build:dnas && hc app pack ./workdir",
    "build:dnas": "npm run build:zomes${mergeStrings(
      happ.dnas.map(dna => ` && hc dna pack ./${getDnaPath(happ, dna.name)}workdir`),
    )}",
    "build:zomes": "CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown",
    "playground": "run-singleton \\"holochain-playground\\""
  },
  "devDependencies": {
    "@holochain-playground/cli": "^0.0.8",
    "concurrently": "^6.2.1",
    "concurrently-repeat": "^0.0.1",
    "cross-env": "^7.0.3",
    "new-port-cli": "^1.0.0",
    "rimraf": "^3.0.2",
    "run-singleton-cli": "^0.0.5",
    "bestzip": "^2.2.0"
  },
  "engines": {
    "npm": ">=7.0.0"
  }
}
`,
});
