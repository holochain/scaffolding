import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const kebabSingularDetailTs = ({kebabSingular_, moduleNameTitleCase}: {kebabSingular_: string; moduleNameTitleCase: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `import { customElement } from 'lit/decorators.js';
import { ${moduleNameTitleCase}Detail } from '../elements/${kebabSingular_}detail';

@customElement('${kebabSingular_}detail')
class PD extends ${moduleNameTitleCase}Detail {}
`
});
    