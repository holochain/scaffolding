import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const contextTs = ({moduleNameSnakeCase, moduleNamePluralTitleCase, kebabPlural_, moduleNamePlural}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; kebabPlural_: string; moduleNamePlural: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `import { Context, createContext } from '@holochain-open-dev/context';
import { ${moduleNamePluralTitleCase}Store } from './${kebabPlural_}store';

export const ${camelCase(moduleNamePlural)}StoreContext: Context<${moduleNamePluralTitleCase}Store> = createContext(
  'hc_zome${moduleNameSnakeCase}s/store'
);
`
});
    