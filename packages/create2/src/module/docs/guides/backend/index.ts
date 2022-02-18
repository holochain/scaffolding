import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { indexMd } from './indexMd';
import { typesMd } from './typesMd';
import { zomeMd } from './zomeMd';  

export default ({moduleNameSnakeCase, moduleNamePlural}: {moduleNameSnakeCase: string; moduleNamePlural: string;}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'index.md': indexMd(),
  'types.md': typesMd({moduleNameSnakeCase, moduleNamePlural}),
  'zome.md': zomeMd({moduleNameSnakeCase})
  }
})