import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import joiningBlocks from './joiningBlocks';
import partials from './partials';  

export default (): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  '_joiningBlocks': joiningBlocks(),
  'partials': partials()
  }
})