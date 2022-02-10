import { PatcherNodeType } from '@patcher/types'; 

import { faviconIco } from './faviconIco';
import { indexHtml } from './indexHtml';  

export default () => ({
  type: PatcherNodeType.Directory,
  children: {
  'favicon.ico': faviconIco(),
  'index.html': indexHtml()
  }
})