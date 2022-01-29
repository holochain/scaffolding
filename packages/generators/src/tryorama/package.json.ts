export default (tryoramaVersion: string) =>
  `{
  "name": "tests",
  "version": "0.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "test": "set -o pipefail && TRYORAMA_LOG_LEVEL=info RUST_BACKTRACE=1 RUST_LOG=holochain::core::ribosome::host_fn::debug=debug TRYORAMA_HOLOCHAIN_PATH=\\"holochain\\" node --loader ts-node/esm --experimental-specifier-resolution=node src/index.ts | tap-diff"
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
    "ts-node": "^10.4.0",
    "typescript": "4.3.5",
    "uuidv4": "^6.2.11"
  },
  "devDependencies": {
    "@holochain/tryorama": "${tryoramaVersion}",
    "@types/lodash": "^4.14.158",
    "@types/node": "^14.0.14",
    "@detools/tap-diff": "^0.2.2",
    "tap-diff": "^0.1.1"
  },
  "type": "module"
}
`;
