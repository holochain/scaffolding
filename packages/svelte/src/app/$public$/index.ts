import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { faviconPng } from './faviconPng';
import { globalCss } from './globalCss';
import { indexHtml } from './indexHtml';  

export default (): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'favicon.png': faviconPng(),
  'global.css': globalCss(),
  'index.html': indexHtml()
  }
})