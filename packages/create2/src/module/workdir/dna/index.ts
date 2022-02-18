import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { dnaYaml } from './dnaYaml';  

export default ({moduleNameSnakeCase, kebabPlural_, moduleNamePlural}: {moduleNameSnakeCase: string; kebabPlural_: string; moduleNamePlural: string;}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'dna.yaml': dnaYaml({moduleNameSnakeCase, kebabPlural_, moduleNamePlural})
  }
})