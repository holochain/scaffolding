import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { eleventyignore } from './eleventyignore';
import { nojekyll } from './nojekyll';
import assets from './assets';
import data from './data';
import includes from './includes';
import guides from './guides';
import { indexMd } from './indexMd';  

export default ({moduleNameSnakeCase, moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName, packageName}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; _kebab: string; kebabPlural_: string; kebabSingular_: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string; packageName: string;}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  '.eleventyignore': eleventyignore(),
  '.nojekyll': nojekyll(),
  '_assets': assets({moduleNameSnakeCase, moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName}),
  '_data': data({packageName, moduleNamePlural}),
  '_includes': includes(),
  'guides': guides({moduleNameSnakeCase, moduleNamePlural, packageName, moduleNamePluralTitleCase, _kebab, kebabSingular_, moduleName, moduleNameTitleCase, kebabPlural_}),
  'index.md': indexMd({packageName, moduleNamePlural})
  }
})