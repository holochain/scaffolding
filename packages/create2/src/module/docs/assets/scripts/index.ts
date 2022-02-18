import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { registerServiceWorkerJs } from './registerServiceWorkerJs';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'registerServiceWorker.js': registerServiceWorkerJs()
  }
})