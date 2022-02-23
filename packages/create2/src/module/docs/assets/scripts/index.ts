import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { registerServiceWorkerJs } from './registerServiceWorkerJs';  

export default (): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'registerServiceWorker.js': registerServiceWorkerJs()
  }
})