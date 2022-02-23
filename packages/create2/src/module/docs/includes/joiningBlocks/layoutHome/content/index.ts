import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { $49$0HeroNjk } from './$49$0HeroNjk';  

export default (): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  '10-hero.njk': $49$0HeroNjk()
  }
})