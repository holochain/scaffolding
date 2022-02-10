import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { faviconIco } from './faviconIco';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'favicon.ico': faviconIco()
  }
})