import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import workflows from './workflows';  

export default (): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'workflows': workflows()
  }
})