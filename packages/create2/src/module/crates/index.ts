import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import types from './types';
import zome from './zome';  

export default ({moduleNameSnakeCase, moduleNamePlural, moduleNamePluralTitleCase, moduleNameTitleCase, moduleName}: {moduleNameSnakeCase: string; moduleNamePlural: string; moduleNamePluralTitleCase: string; moduleNameTitleCase: string; moduleName: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'types': types({moduleNameSnakeCase, moduleNamePlural, moduleNamePluralTitleCase, moduleNameTitleCase, moduleName}),
  'zome': zome({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNamePlural, moduleNameTitleCase, moduleName})
  }
})