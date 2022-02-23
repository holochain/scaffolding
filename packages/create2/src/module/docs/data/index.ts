import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { footerJson } from './footerJson';
import { siteCjs } from './siteCjs';  

export default ({packageName, moduleNamePlural}: {packageName: string; moduleNamePlural: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'footer.json': footerJson(),
  'site.cjs': siteCjs({packageName, moduleNamePlural})
  }
})