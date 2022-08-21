import { HappDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';

import { getDnaPath, getUiPackageName, mergeStrings } from '../utils';

export const rootPackageJson = (happ: HappDefinition): ScFile => ({
  type: ScNodeType.File,
  content: `{
  "name": "${happ.name}-dev",
  "private": true,
  "workspaces": [
    "ui",
    "tests"
  ],
  "scripts": {
    "start": "npm run network 2",
    "network": "hc s clean && npm run build:happ && concurrently-repeat \\"npm run start:agent\\"",
    "start:agent": "cross-env HC_PORT=$(port) concurrently -k \\"npm run start:happ\\" \\"sleep 5 && npm run start -w ${getUiPackageName(
      happ,
    )}\\"",
    "test": "npm run build:happ && npm t -w tests",
    "start:happ": "concurrently \\"RUST_LOG=warn echo \"pass\" | hc s --piped generate ./workdir/${happ.name}.happ --run=$HC_PORT -a ${
    happ.name
  } network mdns\\" \\"npm run playground\\"",
    "package": "npm run build:happ && npm run package -w ui && hc web-app pack workdir",
    "build:happ": "npm run build:dnas && hc app pack ./workdir",
    "build:dnas": "npm run build:zomes${mergeStrings(
      happ.dnas.map(dna => ` && hc dna pack ./${getDnaPath(happ, dna.name)}workdir`),
    )}",
    "build:zomes": "CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown",
    "playground": "run-singleton \\"holochain-playground\\""
  },
  "devDependencies": {
    "@holochain-playground/cli": "^0.0.11",
    "concurrently": "^6.2.1",
    "concurrently-repeat": "^0.0.1",
    "cross-env": "^7.0.3",
    "new-port-cli": "^1.0.0",
    "rimraf": "^3.0.2",
    "run-singleton-cli": "^0.0.5"
  },
  "engines": {
    "npm": ">=7.0.0"
  }
}
`,
});
