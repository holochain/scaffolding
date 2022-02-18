import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import layoutHome from './layoutHome';
import head from './head';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  '_layoutHome': layoutHome(),
  'head': head()
  }
})