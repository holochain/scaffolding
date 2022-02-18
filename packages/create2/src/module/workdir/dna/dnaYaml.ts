import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const dnaYaml = ({moduleNameSnakeCase, kebabPlural_, moduleNamePlural}: {moduleNameSnakeCase: string; kebabPlural_: string; moduleNamePlural: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `---
manifest_version: "1"
name: ${kebabPlural_}test
uuid: 00000000-0000-0000-0000-000000000000
properties: ~
zomes:
  - name: ${snakeCase(moduleNamePlural)}
    bundled: ../../target/wasm32-unknown-unknown/release/hc_zome${moduleNameSnakeCase}s.wasm
`
});
    