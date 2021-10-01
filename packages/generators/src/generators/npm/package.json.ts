import { getDnaPath, getUiPackageName, mergeStrings } from '../utils';
import { HappDefinition } from '../../types';

export default ( happ: HappDefinition) => 
`{
  "name": "${happ.name}-dev",
  "private": true,
  "workspaces": [
    "ui",
    "tests"
  ],
  "scripts": {
    "start": "npm run build:happ && cross-env HC_PORT=$(port) concurrently -k \\"npm run start:happ\\" \\"npm run start -w ${getUiPackageName(happ)}\\"",
    "test": "npm run build:happ && npm t -w tests",
    "start:happ": "hc sandbox clean && RUST_LOG=warn hc s generate ./workdir/${happ.name}.happ --run=$HC_PORT -a ${
    happ.name
  } network mdns",
    "package": "npm run build:happ && npm run package:ui && hc web-app pack workdir",
    "package:ui": "npm run build -w ${getUiPackageName(happ)} && cd ui/dist && bestzip ../dist.zip *",
    "build:happ": "npm run build:dnas && hc app pack ./workdir",
    "build:dnas": "npm run build:zomes${mergeStrings(happ.dnas.map(dna => ` && hc dna pack ./${getDnaPath(happ, dna.name)}workdir`))}",
    "build:zomes": "CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown"
  },
  "devDependencies": {
    "concurrently": "^6.2.1",
    "cross-env": "^7.0.3",
    "new-port-cli": "^1.0.0",
    "rimraf": "^3.0.2",
    "bestzip": "^2.2.0"
  }
}
`