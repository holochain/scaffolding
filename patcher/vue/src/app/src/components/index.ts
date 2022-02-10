import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { helloWorldVue } from './helloWorldVue';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'HelloWorld.vue': helloWorldVue()
  }
})