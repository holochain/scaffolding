import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { handlersRs } from './handlersRs';
import { libRs } from './libRs';
import { utilsRs } from './utilsRs';  

export default ({moduleNameSnakeCase, moduleNameTitleCase, moduleName, moduleNamePluralTitleCase, moduleNamePlural}: {moduleNameSnakeCase: string; moduleNameTitleCase: string; moduleName: string; moduleNamePluralTitleCase: string; moduleNamePlural: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'handlers.rs': handlersRs({moduleNameSnakeCase, moduleNameTitleCase, moduleName}),
  'lib.rs': libRs({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase, moduleNamePlural, moduleName}),
  'utils.rs': utilsRs()
  }
})