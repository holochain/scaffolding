import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { handlersRs } from './handlersRs';
import { libRs } from './libRs';
import { utilsRs } from './utilsRs';  

export default ({moduleNameSnakeCase, moduleNameTitleCase, moduleName, moduleNamePluralTitleCase, moduleNamePlural}: {moduleNameSnakeCase: string; moduleNameTitleCase: string; moduleName: string; moduleNamePluralTitleCase: string; moduleNamePlural: string;}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'handlers.rs': handlersRs({moduleNameSnakeCase, moduleNameTitleCase, moduleName}),
  'lib.rs': libRs({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase, moduleNamePlural, moduleName}),
  'utils.rs': utilsRs()
  }
})