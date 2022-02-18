import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { indexHtml } from './indexHtml';  

export default ({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural}: {moduleNamePluralTitleCase: string; _kebab: string; kebabPlural_: string; kebabSingular_: string; moduleNameTitleCase: string; moduleNamePlural: string;}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'index.html': indexHtml({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural})
  }
})