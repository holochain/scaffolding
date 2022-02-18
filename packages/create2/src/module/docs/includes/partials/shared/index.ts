import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { logoLinkNjk } from './logoLinkNjk';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'logoLink.njk': logoLinkNjk()
  }
})