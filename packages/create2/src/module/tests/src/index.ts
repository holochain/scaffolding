import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { indexTs } from './indexTs';  

export default ({testZomeName, moduleNameSnakeCase, moduleNamePluralTitleCase, kebabPlural_, moduleNameTitleCase, moduleNamePlural, moduleName}: {testZomeName: string; moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; kebabPlural_: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'index.ts': indexTs({testZomeName, moduleNameSnakeCase, moduleNamePluralTitleCase, kebabPlural_, moduleNameTitleCase, moduleNamePlural, moduleName})
  }
})