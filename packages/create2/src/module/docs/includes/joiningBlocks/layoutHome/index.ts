import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import content from './content';  

export default (): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'content': content()
  }
})