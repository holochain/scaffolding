import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { footerJson } from './footerJson';
import { siteCjs } from './siteCjs';  

export default ({packageName, moduleNamePlural}: {packageName: string; moduleNamePlural: string;}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'footer.json': footerJson(),
  'site.cjs': siteCjs({packageName, moduleNamePlural})
  }
})