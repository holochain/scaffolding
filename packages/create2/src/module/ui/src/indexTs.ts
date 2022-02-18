import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const indexTs = ({_kebab, kebabPlural_, kebabSingular_}: {_kebab: string; kebabPlural_: string; kebabSingular_: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `export * from './types';
export * from './context';
export * from './elements/create${_kebab}';
export * from './elements/update${_kebab}';
export * from './elements/my${_kebab}';
export * from './elements/search-agent';
export * from './elements/${kebabSingular_}prompt';
export * from './elements/list${_kebab}s';
export * from './elements/agent-avatar';
export * from './elements/holo-identicon';
export * from './elements/${kebabSingular_}detail';
export * from './${kebabPlural_}service';
export * from './${kebabPlural_}store';
export * from './config';
`
});
    