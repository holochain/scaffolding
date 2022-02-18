import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { cargoToml } from './cargoToml';
import { readmeMd } from './readmeMd';
import src from './src';  

export default ({moduleNameSnakeCase, moduleNamePlural, moduleNamePluralTitleCase, moduleNameTitleCase, moduleName}: {moduleNameSnakeCase: string; moduleNamePlural: string; moduleNamePluralTitleCase: string; moduleNameTitleCase: string; moduleName: string;}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'Cargo.toml': cargoToml({moduleNameSnakeCase, moduleNamePlural}),
  'README.md': readmeMd({moduleNameSnakeCase, moduleNamePlural}),
  'src': src({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase, moduleName})
  }
})