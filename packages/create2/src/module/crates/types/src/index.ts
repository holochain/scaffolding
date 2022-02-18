import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { libRs } from './libRs';  

export default ({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase, moduleName}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; moduleNameTitleCase: string; moduleName: string;}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'lib.rs': libRs({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase, moduleName})
  }
})