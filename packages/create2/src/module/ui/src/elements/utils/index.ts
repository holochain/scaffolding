import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { imageTs } from './imageTs';
import { sharedStylesTs } from './sharedStylesTs';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'image.ts': imageTs(),
  'shared-styles.ts': sharedStylesTs()
  }
})