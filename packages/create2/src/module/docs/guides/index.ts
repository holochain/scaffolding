import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import backend from './backend';
import frontend from './frontend';
import { indexMd } from './indexMd';
import settingUp from './settingUp';  

export default ({moduleNameSnakeCase, moduleNamePlural, packageName, moduleNamePluralTitleCase, _kebab, kebabSingular_, moduleName, moduleNameTitleCase, kebabPlural_}: {moduleNameSnakeCase: string; moduleNamePlural: string; packageName: string; moduleNamePluralTitleCase: string; _kebab: string; kebabSingular_: string; moduleName: string; moduleNameTitleCase: string; kebabPlural_: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'backend': backend({moduleNameSnakeCase, moduleNamePlural}),
  'frontend': frontend({packageName, moduleNamePluralTitleCase, _kebab, kebabSingular_, moduleNamePlural, moduleName, moduleNameSnakeCase, moduleNameTitleCase, kebabPlural_}),
  'index.md': indexMd({moduleNameTitleCase, moduleNamePlural, moduleName}),
  'setting-up': settingUp({packageName, moduleNamePluralTitleCase, _kebab, moduleNameTitleCase, moduleNamePlural, moduleNameSnakeCase})
  }
})