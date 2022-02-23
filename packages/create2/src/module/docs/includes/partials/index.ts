import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import shared from './shared';  

export default (): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  '_shared': shared()
  }
})