import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { faviconIco } from './faviconIco';  

export default (): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'favicon.ico': faviconIco()
  }
})