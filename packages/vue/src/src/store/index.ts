import { PatcherNodeType } from '@patcher/types'; 

import { holochainTs } from './holochainTs';
import { indexTs } from './indexTs';  

export default () => ({
  type: PatcherNodeType.Directory,
  children: {
  'holochain.ts': holochainTs(),
  'index.ts': indexTs()
  }
})