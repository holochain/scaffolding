import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const zomeMd = ({moduleNameSnakeCase}: {moduleNameSnakeCase: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `# Backend Docs >> hc_zome${moduleNameSnakeCase}s ||10

Use this crate directly if you want include and maybe extend this zome in your DNA.

Notice that just by importing this crate, all its zome functions will be automatically defined in the consuming crate. This could create collisions in function names or entry definitions.

Read the documentation for the zome functions available from this zome:

- https://docs.rs/hc_zome${moduleNameSnakeCase}s`
});
    