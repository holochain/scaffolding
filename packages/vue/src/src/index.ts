import { PatcherNodeType } from '@patcher/types'; 

import { appVue } from './appVue';
import assets from './assets';
import components from './components';
import { mainTs } from './mainTs';
import router from './router';
import { shimsVueDTs } from './shimsVueDTs';
import store from './store';
import views from './views';  

export default ({zomeName, fnName, payload, cellRole}: {zomeName: string; fnName: string; payload: string; cellRole: string;}) => ({
  type: PatcherNodeType.Directory,
  children: {
  'App.vue': appVue(),
  'assets': assets(),
  'components': components({zomeName, fnName, payload, cellRole}),
  'main.ts': mainTs(),
  'router': router(),
  'shims-vue.d.ts': shimsVueDTs(),
  'store': store(),
  'views': views()
  }
})