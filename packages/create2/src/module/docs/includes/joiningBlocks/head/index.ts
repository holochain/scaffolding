import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { $49$0DefaultsNjk } from './$49$0DefaultsNjk';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  '10-defaults.njk': $49$0DefaultsNjk()
  }
})