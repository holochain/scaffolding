import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const myKebabTs = ({_kebab, moduleNameTitleCase}: {_kebab: string; moduleNameTitleCase: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `import { customElement } from 'lit/decorators.js';
import { My${moduleNameTitleCase} } from '../elements/my${_kebab}';

@customElement('my${_kebab}')
class MP extends My${moduleNameTitleCase} {}
`
});
    