import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const packageJson = ({packageName, kebabPlural_}: {packageName: string; kebabPlural_: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `{
  "name": "${packageName}-dev",
  "private": true,
  "workspaces": [
    "ui",
    "tests"
  ],
  "scripts": {
    "start": "npm run build:happ && npm run start:agent",
    "network": "npm run build:happ && concurrently-repeat \\"npm run start:agent\\"",
    "start:agent": "cross-env HC_PORT=\$(port) concurrently \\"npm run playground\\" \\"npm run start:happ\\" \\"npm run start -w ${packageName}\\"",
    "test": "npm run build:happ && npm t -w ${packageName}-test",
    "start:happ": "RUST_LOG=warn hc s generate ./workdir/happ/${kebabPlural_}test.happ --run=\$HC_PORT network  --bootstrap https://bootstrap-staging.holo.host/ quic",
    "build:happ": "npm run build:dna && hc app pack workdir/happ",
    "build:dna": "npm run build:zome && hc dna pack workdir/dna",
    "build:zome": "CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown",
    "playground": "run-singleton \\"holochain-playground\\"",
    "docs:setup": "npm run -w ${packageName} analyze && cp ui/custom-elements.json docs/_assets/_static",
    "docs:start": "npm run docs:setup && rocket start",
    "docs:publish": "npm run docs:setup && rocket build && gh-pages --dotfiles -d ./_site -b gh-pages"
  },
  "devDependencies": {
    "@custom-elements-manifest/to-markdown": "^0.1.0",
    "@holochain-playground/cli": "^0.0.8",
    "@rocket/cli": "^0.10.0",
    "@rocket/launch": "^0.6.0",
    "@rollup/plugin-commonjs": "^21.0.1",
    "@rollup/plugin-replace": "^3.0.1",
    "@scoped-elements/markdown-renderer": "^0.0.3",
    "api-viewer-element": "^1.0.0-pre.4",
    "concurrently": "^6.2.1",
    "concurrently-repeat": "^0.0.1",
    "cross-env": "^7.0.3",
    "gh-pages": "^3.2.3",
    "new-port-cli": "^1.0.0",
    "run-singleton-cli": "^0.0.5"
  },
  "type": "module"
}
`
});
    