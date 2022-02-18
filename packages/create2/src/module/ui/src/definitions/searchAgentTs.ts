import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const searchAgentTs = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `import { customElement } from 'lit/decorators.js';
import { SearchAgent } from '../elements/search-agent';

@customElement('search-agent')
class SA extends SearchAgent {}
`
});
    