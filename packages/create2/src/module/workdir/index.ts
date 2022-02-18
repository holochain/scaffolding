import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import dna from './dna';
import happ from './happ';  

export default ({moduleNameSnakeCase, kebabPlural_, moduleNamePlural}: {moduleNameSnakeCase: string; kebabPlural_: string; moduleNamePlural: string;}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'dna': dna({moduleNameSnakeCase, kebabPlural_, moduleNamePlural}),
  'happ': happ({kebabPlural_, moduleNamePlural})
  }
})