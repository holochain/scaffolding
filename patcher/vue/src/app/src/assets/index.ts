import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { logoPng } from './logoPng';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'logo.png': logoPng()
  }
})