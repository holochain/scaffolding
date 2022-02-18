import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const kebabSingularPromptTs = ({kebabSingular_, moduleNameTitleCase}: {kebabSingular_: string; moduleNameTitleCase: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `import { customElement } from 'lit/decorators.js';
import { ${moduleNameTitleCase}Prompt } from '../elements/${kebabSingular_}prompt';

@customElement('${kebabSingular_}prompt')
class PP extends ${moduleNameTitleCase}Prompt {}
`
});
    