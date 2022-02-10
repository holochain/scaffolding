import { PatcherNodeType } from '@patcher/types'; 

import { logoPng } from './logoPng';  

export default () => ({
  type: PatcherNodeType.Directory,
  children: {
  'logo.png': logoPng()
  }
})