import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import dna from './dna';
import happ from './happ';  

export default ({moduleNameSnakeCase, kebabPlural_, moduleNamePlural}: {moduleNameSnakeCase: string; kebabPlural_: string; moduleNamePlural: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'dna': dna({moduleNameSnakeCase, kebabPlural_, moduleNamePlural}),
  'happ': happ({kebabPlural_, moduleNamePlural})
  }
})