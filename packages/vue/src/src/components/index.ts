import { PatcherNodeType } from '@patcher/types'; 

import { helloWorldVue } from './helloWorldVue';  

export default ({zomeName, fnName, payload, cellRole}: {zomeName: string; fnName: string; payload: string; cellRole: string;}) => ({
  type: PatcherNodeType.Directory,
  children: {
  'HelloWorld.vue': helloWorldVue({zomeName, fnName, payload, cellRole})
  }
})