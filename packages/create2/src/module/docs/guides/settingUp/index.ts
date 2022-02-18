import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { addingTheFrontendMd } from './addingTheFrontendMd';
import { addingTheZomeMd } from './addingTheZomeMd';
import { indexMd } from './indexMd';  

export default ({packageName, moduleNamePluralTitleCase, _kebab, moduleNameTitleCase, moduleNamePlural, moduleNameSnakeCase}: {packageName: string; moduleNamePluralTitleCase: string; _kebab: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleNameSnakeCase: string;}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'adding-the-frontend.md': addingTheFrontendMd({packageName, moduleNamePluralTitleCase, _kebab, moduleNameTitleCase, moduleNamePlural}),
  'adding-the-zome.md': addingTheZomeMd({moduleNameSnakeCase, moduleNamePlural}),
  'index.md': indexMd()
  }
})