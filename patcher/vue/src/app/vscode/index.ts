import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { extensionsJson } from './extensionsJson';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'extensions.json': extensionsJson()
  }
})