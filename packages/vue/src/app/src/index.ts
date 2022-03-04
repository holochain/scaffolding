import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { appVue } from './appVue';
import assets from './assets';
import { envDTs } from './envDTs';
import { mainTs } from './mainTs';  

export default ({happName, appContent, appSubcomponents, subcomponentsImports}: {happName: string; appContent: string; appSubcomponents: string; subcomponentsImports: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'App.vue': appVue({happName, appContent, appSubcomponents, subcomponentsImports}),
  'assets': assets(),
  'env.d.ts': envDTs(),
  'main.ts': mainTs()
  }
})