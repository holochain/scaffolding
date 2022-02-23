import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { imageTs } from './imageTs';
import { sharedStylesTs } from './sharedStylesTs';  

export default (): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'image.ts': imageTs(),
  'shared-styles.ts': sharedStylesTs()
  }
})