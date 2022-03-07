import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const gitignore = (): ScFile => ({
  type: ScNodeType.File,
  content: `/node_modules/
/public/build/

.DS_Store
`
});
    