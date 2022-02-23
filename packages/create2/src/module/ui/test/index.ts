import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { elementsTestJs } from './elementsTestJs';
import mocks from './mocks';  

export default ({_kebab, moduleNameTitleCase, moduleName, moduleNamePluralTitleCase, moduleNamePlural, moduleNameSnakeCase}: {_kebab: string; moduleNameTitleCase: string; moduleName: string; moduleNamePluralTitleCase: string; moduleNamePlural: string; moduleNameSnakeCase: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'elements.test.js': elementsTestJs({_kebab, moduleNameTitleCase, moduleName}),
  'mocks': mocks({moduleNamePluralTitleCase, moduleNamePlural, moduleNameSnakeCase, moduleName})
  }
})