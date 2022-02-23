import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { happYaml } from './happYaml';  

export default ({kebabPlural_, moduleNamePlural}: {kebabPlural_: string; moduleNamePlural: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'happ.yaml': happYaml({kebabPlural_, moduleNamePlural})
  }
})