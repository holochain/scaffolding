import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import content from './content';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'content': content()
  }
})