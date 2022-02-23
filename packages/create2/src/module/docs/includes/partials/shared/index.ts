import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { logoLinkNjk } from './logoLinkNjk';  

export default (): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'logoLink.njk': logoLinkNjk()
  }
})