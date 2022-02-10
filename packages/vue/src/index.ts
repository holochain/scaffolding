import { PatcherNodeType } from '@patcher/types'; 

import { browserslistrc } from './browserslistrc';
import { eslintrcJs } from './eslintrcJs';
import { gitignore } from './gitignore';
import { readmeMd } from './readmeMd';
import { babelConfigJs } from './babelConfigJs';
import { packageJson } from './packageJson';
import $public$ from './public';
import src from './src';
import { tsconfigJson } from './tsconfigJson';
import { vueConfigJs } from './vueConfigJs';  

export default ({zomeName, fnName, payload, cellRole}: {zomeName: string; fnName: string; payload: string; cellRole: string;}) => ({
  type: PatcherNodeType.Directory,
  children: {
  '.browserslistrc': browserslistrc(),
  '.eslintrc.js': eslintrcJs(),
  '.gitignore': gitignore(),
  'README.md': readmeMd(),
  'babel.config.js': babelConfigJs(),
  'package.json': packageJson(),
  'public': $public$(),
  'src': src({zomeName, fnName, payload, cellRole}),
  'tsconfig.json': tsconfigJson(),
  'vue.config.js': vueConfigJs()
  }
})