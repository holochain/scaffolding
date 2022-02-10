import { PatcherNodeType } from '@patcher/types'; 

import { indexTs } from './indexTs';  

export default () => ({
  type: PatcherNodeType.Directory,
  children: {
  'index.ts': indexTs()
  }
})