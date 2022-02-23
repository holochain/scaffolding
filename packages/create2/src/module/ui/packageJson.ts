import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const packageJson = ({packageName, moduleNameSnakeCase, _kebab, kebabSingular_}: {packageName: string; moduleNameSnakeCase: string; _kebab: string; kebabSingular_: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `{
  "name": "${packageName}",
  "version": "0.0.8",
  "description": "Frontend module for the Holochain hc_zome${moduleNameSnakeCase}s zome",
  "author": "guillem.cordoba@gmail.com",
  "license": "MIT",
  "main": "dist/index.js",
  "module": "dist/index.js",
  "exports": {
    ".": "./dist/index.js",
    "./create${_kebab}": "./dist/definitions/create${_kebab}.js",
    "./update${_kebab}": "./dist/definitions/update${_kebab}.js",
    "./my${_kebab}": "./dist/definitions/my${_kebab}.js",
    "./${kebabSingular_}detail": "./dist/definitions/${kebabSingular_}detail.js",
    "./${kebabSingular_}prompt": "./dist/definitions/${kebabSingular_}prompt.js",
    "./search-agent": "./dist/definitions/search-agent.js",
    "./agent-avatar": "./dist/definitions/agent-avatar.js",
    "./holo-identicon": "./dist/definitions/holo-identicon.js",
    "./list${_kebab}s": "./dist/definitions/list${_kebab}s.js",
    "./mocks": "./dist/mocks.js"
  },
  "files": [
    "dist"
  ],
  "scripts": {
    "start": "npm run build && concurrently -k --names -k \\"npm run build-watch\\" \\"web-dev-server --config web-dev-server.config.mjs\\"",
    "build": "rimraf dist && tsc",
    "build-watch": "tsc --watch --preserveWatchOutput --incremental",
    "test": "npm run build && web-test-runner --coverage",
    "test-debug": "npm run build && DEBUG=true web-test-runner --coverage",
    "test-watch": "web-test-runner --watch",
    "e2e": "CONDUCTOR_URL=ws://localhost:8888 concurrently -k -s first \\"npm:test\\" \\"npm:start-holochain\\"",
    "e2e-debug": "DEBUG=true npm run e2e",
    "lint": "eslint --ext .ts,.html . --ignore-path .gitignore",
    "analyze": "cem analyze --litelement --exclude dist",
    "format": "eslint --ext .ts,.html . --fix --ignore-path .gitignore",
    "publish-to-branch": "npm run lint && npm run build && rimraf node_modules && gh-pages -d ./ -b ui-build && cd .. && npm i"
  },
  "dependencies": {
    "@holo-host/identicon": "^0.1.0",
    "@holochain-open-dev/cell-client": "^0.3.2",
    "@holochain/client": "^0.3.2",
    "@holochain-open-dev/core-types": "^0.2.0",
    "@holochain-open-dev/context": "^0.0.3",
    "@open-wc/scoped-elements": "^2.0.1",
    "@scoped-elements/material-web": "0.0.16",
    "@scoped-elements/shoelace": "0.0.8",
    "lit": "^2.1.1",
    "lit-svelte-stores": "^0.1.3",
    "lodash-es": "^4.17.21",
    "svelte": "^3.42.4"
  },
  "devDependencies": {
    "@custom-elements-manifest/analyzer": "^0.5.7",
    "@mdjs/mdjs-preview": "^0.5.6",
    "@open-wc/eslint-config": "^2.0.0",
    "@open-wc/testing": "^3.0.0-next.5",
    "@open-wc/testing-karma": "^4.0.9",
    "@rollup/plugin-commonjs": "^15.1.0",
    "@rollup/plugin-replace": "^2.3.3",
    "@types/lodash-es": "^4.17.5",
    "@types/node": "13.11.1",
    "@typescript-eslint/eslint-plugin": "^2.20.0",
    "@typescript-eslint/parser": "^2.20.0",
    "@web/dev-server": "^0.1.0",
    "@web/dev-server-rollup": "^0.3.0",
    "@web/test-runner": "^0.11.0",
    "@web/test-runner-puppeteer": "^0.8.0",
    "concurrently": "^5.1.0",
    "deepmerge": "^3.2.0",
    "eslint": "^6.1.0",
    "eslint-config-prettier": "^6.11.0",
    "gh-pages": "~3.1.0",
    "husky": "^7.0.2",
    "lint-staged": "^10.0.0",
    "prettier": "^2.0.4",
    "rimraf": "^3.0.2",
    "tslib": "^2.0.0",
    "typescript": "4.3.5"
  },
  "eslintConfig": {
    "extends": [
      "@open-wc/eslint-config",
      "eslint-config-prettier"
    ]
  },
  "publishConfig": {
    "access": "public"
  },
  "prettier": {
    "singleQuote": true,
    "arrowParens": "avoid"
  },
  "customElements": "custom-elements.json",
  "type": "module"
}
`
});
    