import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const huskyrc = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `{
  "hooks": {
    "pre-commit": "cd ui; npm run format"
  }
}`
});
    