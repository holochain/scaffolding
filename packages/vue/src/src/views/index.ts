import { PatcherNodeType } from '@patcher/types'; 

import { aboutVue } from './aboutVue';
import { homeVue } from './homeVue';  

export default () => ({
  type: PatcherNodeType.Directory,
  children: {
  'About.vue': aboutVue(),
  'Home.vue': homeVue()
  }
})