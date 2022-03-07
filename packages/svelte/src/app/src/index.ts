import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { appSvelte } from './appSvelte';
import { contextsTs } from './contextsTs';
import { globalDTs } from './globalDTs';
import { mainTs } from './mainTs';  

export default ({happName, subcomponentImports, appContent}: {happName: string; subcomponentImports: string; appContent: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'App.svelte': appSvelte({happName, subcomponentImports, appContent}),
  'contexts.ts': contextsTs(),
  'global.d.ts': globalDTs(),
  'main.ts': mainTs()
  }
})