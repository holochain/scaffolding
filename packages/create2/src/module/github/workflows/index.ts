import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { mainYml } from './mainYml';  

export default (): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'main.yml': mainYml()
  }
})