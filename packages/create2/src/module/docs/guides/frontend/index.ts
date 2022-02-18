import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { elementsMd } from './elementsMd';
import { indexMd } from './indexMd';
import { kebabPluralServiceMd } from './kebabPluralServiceMd';
import { kebabPluralStoreMd } from './kebabPluralStoreMd';  

export default ({packageName, moduleNamePluralTitleCase, _kebab, kebabSingular_, moduleNamePlural, moduleName, moduleNameSnakeCase, moduleNameTitleCase, kebabPlural_}: {packageName: string; moduleNamePluralTitleCase: string; _kebab: string; kebabSingular_: string; moduleNamePlural: string; moduleName: string; moduleNameSnakeCase: string; moduleNameTitleCase: string; kebabPlural_: string;}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'elements.md': elementsMd({packageName, moduleNamePluralTitleCase, _kebab, kebabSingular_, moduleNamePlural, moduleName}),
  'index.md': indexMd(),
  [`${kebabPlural_}service.md`]: kebabPluralServiceMd({packageName, moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase}),
  [`${kebabPlural_}store.md`]: kebabPluralStoreMd({packageName, moduleNamePluralTitleCase, moduleNamePlural, moduleName})
  }
})