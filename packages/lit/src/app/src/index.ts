import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { contextsTs } from './contextsTs';
import { holochainAppTs } from './holochainAppTs';  

export default ({happName}: {happName: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'contexts.ts': contextsTs(),
  'holochain-app.ts': holochainAppTs({happName})
  }
})