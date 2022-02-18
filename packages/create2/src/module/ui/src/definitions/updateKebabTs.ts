import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const updateKebabTs = ({_kebab, moduleNameTitleCase}: {_kebab: string; moduleNameTitleCase: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `import { customElement } from 'lit/decorators.js';
import { Update${moduleNameTitleCase} } from '../elements/update${_kebab}';

@customElement('update${_kebab}')
class UP extends Update${moduleNameTitleCase} {}
`
});
    