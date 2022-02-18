import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import shared from './shared';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  '_shared': shared()
  }
})