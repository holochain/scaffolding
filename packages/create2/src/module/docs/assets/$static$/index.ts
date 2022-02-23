import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { customElementsJson } from './customElementsJson';  

export default ({moduleNameSnakeCase, moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; _kebab: string; kebabPlural_: string; kebabSingular_: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'custom-elements.json': customElementsJson({moduleNameSnakeCase, moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName})
  }
})