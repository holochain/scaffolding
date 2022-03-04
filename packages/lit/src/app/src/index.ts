import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { contextsTs } from './contextsTs';
import { holochainAppTs } from './holochainAppTs';  

export default ({happName, subcomponentImports, appContent}: {happName: string; subcomponentImports: string; appContent: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'contexts.ts': contextsTs(),
  'holochain-app.ts': holochainAppTs({happName, subcomponentImports, appContent})
  }
})