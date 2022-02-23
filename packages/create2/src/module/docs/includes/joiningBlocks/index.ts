import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import layoutHome from './layoutHome';
import head from './head';  

export default (): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  '_layoutHome': layoutHome(),
  'head': head()
  }
})