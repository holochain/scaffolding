import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { gitignore } from './gitignore';
import { packageJson } from './packageJson';
import src from './src';
import { tsconfigJson } from './tsconfigJson';  

export default ({packageName, testZomeName, moduleNameSnakeCase, moduleNamePluralTitleCase, kebabPlural_, moduleNameTitleCase, moduleNamePlural, moduleName}: {packageName: string; testZomeName: string; moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; kebabPlural_: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  '.gitignore': gitignore(),
  'package.json': packageJson({packageName}),
  'src': src({testZomeName, moduleNameSnakeCase, moduleNamePluralTitleCase, kebabPlural_, moduleNameTitleCase, moduleNamePlural, moduleName}),
  'tsconfig.json': tsconfigJson()
  }
})