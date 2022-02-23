import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { indexHtml } from './indexHtml';  

export default ({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural}: {moduleNamePluralTitleCase: string; _kebab: string; kebabPlural_: string; kebabSingular_: string; moduleNameTitleCase: string; moduleNamePlural: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'index.html': indexHtml({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural})
  }
})