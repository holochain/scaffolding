import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { happYaml } from './happYaml';  

export default ({kebabPlural_, moduleNamePlural}: {kebabPlural_: string; moduleNamePlural: string;}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'happ.yaml': happYaml({kebabPlural_, moduleNamePlural})
  }
})