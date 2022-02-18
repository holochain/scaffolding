import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const gitignore = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `node_modules/
*.log
`
});
    