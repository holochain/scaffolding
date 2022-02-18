import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import $static$ from './$static$';
import { logoSvg } from './logoSvg';
import scripts from './scripts';
import { variablesCss } from './variablesCss';
import { webmanifestJson } from './webmanifestJson';  

export default ({moduleNameSnakeCase, moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; _kebab: string; kebabPlural_: string; kebabSingular_: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string;}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  '_static': $static$({moduleNameSnakeCase, moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName}),
  'logo.svg': logoSvg(),
  'scripts': scripts(),
  'variables.css': variablesCss(),
  'webmanifest.json': webmanifestJson({moduleNamePluralTitleCase, kebabPlural_})
  }
})