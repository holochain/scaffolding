import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { cargoToml } from './cargoToml';
import { readmeMd } from './readmeMd';
import src from './src';  

export default ({moduleNameSnakeCase, moduleNamePlural, moduleNamePluralTitleCase, moduleNameTitleCase, moduleName}: {moduleNameSnakeCase: string; moduleNamePlural: string; moduleNamePluralTitleCase: string; moduleNameTitleCase: string; moduleName: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'Cargo.toml': cargoToml({moduleNameSnakeCase, moduleNamePlural}),
  'README.md': readmeMd({moduleNameSnakeCase, moduleNamePlural}),
  'src': src({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase, moduleName})
  }
})