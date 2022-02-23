import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const packageJson = ({packageName}: {packageName: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `{
  "name": "${packageName}-test",
  "private": true,
  "version": "0.0.0",
  "description": "",
  "main": "index.js",
  "scripts": {
    "test": "TRYORAMA_LOG_LEVEL=info WASM_LOG=warn RUST_LOG=warn RUST_BACKTRACE=1 TRYORAMA_HOLOCHAIN_PATH=\\"holochain\\" ts-node src/index.ts"
  },
  "author": "",
  "license": "ISC",
  "dependencies": {
    "@holochain/tryorama": "0.4.8",
    "@types/lodash": "^4.14.158",
    "@types/node": "^14.0.14",
    "js-base64": "^3.6.0",
    "lodash": "^4.17.19",
    "tape": "^5.0.1",
    "ts-node": "^8.10.2",
    "typescript": "4.3.5"
  }
}
`
});
    