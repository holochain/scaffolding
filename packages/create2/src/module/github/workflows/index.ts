import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { mainYml } from './mainYml';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'main.yml': mainYml()
  }
})