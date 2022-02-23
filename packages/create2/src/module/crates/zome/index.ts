import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { cargoToml } from './cargoToml';
import { readmeMd } from './readmeMd';
import src from './src';  

export default ({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNamePlural, moduleNameTitleCase, moduleName}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; moduleNamePlural: string; moduleNameTitleCase: string; moduleName: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'Cargo.toml': cargoToml({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNamePlural}),
  'README.md': readmeMd({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNamePlural}),
  'src': src({moduleNameSnakeCase, moduleNameTitleCase, moduleName, moduleNamePluralTitleCase, moduleNamePlural})
  }
})