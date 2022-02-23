import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { helloWorldVue } from './helloWorldVue';  

export default (): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'HelloWorld.vue': helloWorldVue()
  }
})