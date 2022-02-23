import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { dnaYaml } from './dnaYaml';  

export default ({moduleNameSnakeCase, kebabPlural_, moduleNamePlural}: {moduleNameSnakeCase: string; kebabPlural_: string; moduleNamePlural: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'dna.yaml': dnaYaml({moduleNameSnakeCase, kebabPlural_, moduleNamePlural})
  }
})