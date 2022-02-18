import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import joiningBlocks from './joiningBlocks';
import partials from './partials';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  '_joiningBlocks': joiningBlocks(),
  'partials': partials()
  }
})