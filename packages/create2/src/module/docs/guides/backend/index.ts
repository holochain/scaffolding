import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { indexMd } from './indexMd';
import { typesMd } from './typesMd';
import { zomeMd } from './zomeMd';  

export default ({moduleNameSnakeCase, moduleNamePlural}: {moduleNameSnakeCase: string; moduleNamePlural: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'index.md': indexMd(),
  'types.md': typesMd({moduleNameSnakeCase, moduleNamePlural}),
  'zome.md': zomeMd({moduleNameSnakeCase})
  }
})