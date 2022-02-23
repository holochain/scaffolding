import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { $49$0DefaultsNjk } from './$49$0DefaultsNjk';  

export default (): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  '10-defaults.njk': $49$0DefaultsNjk()
  }
})