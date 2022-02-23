import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { libRs } from './libRs';  

export default ({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase, moduleName}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; moduleNameTitleCase: string; moduleName: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'lib.rs': libRs({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase, moduleName})
  }
})