import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const readmeMd = ({moduleNameSnakeCase, moduleNamePlural}: {moduleNameSnakeCase: string; moduleNamePlural: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `# hc_zome${moduleNameSnakeCase}s_types

Types for the hc_zome${moduleNameSnakeCase}s zome.

## Documentation

See our [installation instructions and documentation](https://holochain-open-dev.github.io/${moduleNamePlural}).
`
});
    