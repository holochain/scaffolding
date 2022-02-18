import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const typesMd = ({moduleNameSnakeCase, moduleNamePlural}: {moduleNameSnakeCase: string; moduleNamePlural: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `# Backend Docs >> hc_zome${moduleNameSnakeCase}s_types ||10

Use this crate if you want to communicate with the \`hc_zome${moduleNameSnakeCase}s\` zome from an external source, without defining all its zome functions.

This is useful in external zomes that want to do a \`call\` to the ${moduleNamePlural} zome, or in external clients written in rust.

Read the types that are available:

- https://docs.rs/hc_zome${moduleNameSnakeCase}s_types`
});
    