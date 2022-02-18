import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { $49$0HeroNjk } from './$49$0HeroNjk';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  '10-hero.njk': $49$0HeroNjk()
  }
})