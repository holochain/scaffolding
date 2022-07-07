import { ScFile, ScNodeType } from '@source-craft/types';

export const tryoramaPackageJson = (tryoramaVersion: string): ScFile => ({
  type: ScNodeType.File,
  content: `{
  "name": "tests",
  "version": "0.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "test": "TRYORAMA_LOG_LEVEL=info RUST_BACKTRACE=1 RUST_LOG=holochain::core::ribosome::host_fn::debug=debug TRYORAMA_HOLOCHAIN_PATH=\\"holochain\\" node --loader ts-node/esm --experimental-specifier-resolution=node src/index.ts"
  },
  "author": "",
  "license": "CAL-1.0",
  "dependencies": {
    "@msgpack/msgpack": "^2.7.0",
    "esm": "^3.2.25",
    "js-base64": "^3.6.1",
    "lodash-es": "^4.17.21",
    "path": "^0.12.7",
    "tape": "^5.2.2",
    "ts-node": "^10.6.0",
    "typescript": "^4.5.5",
    "uuidv4": "^6.2.11"
  },
  "devDependencies": {
    "@holochain/client": "0.5.0",
    "@holochain/tryorama": "${tryoramaVersion}",
    "@types/lodash": "^4.14.158",
    "@types/node": "^14.0.14",
    "tape-promise": "^4.0.0"
  },
  "type": "module"
}
`,
});
