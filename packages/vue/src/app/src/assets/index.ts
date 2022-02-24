import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { logoPng } from './logoPng';  

export default (): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'logo.png': logoPng()
  }
})