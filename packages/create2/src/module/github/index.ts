import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import workflows from './workflows';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'workflows': workflows()
  }
})