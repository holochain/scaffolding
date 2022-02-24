import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { gitignore } from './gitignore';
import { readmeMd } from './readmeMd';
import { indexHtml } from './indexHtml';
import { packageJson } from './packageJson';
import $public$ from './$public$';
import src from './src';
import { tsconfigJson } from './tsconfigJson';
import { viteConfigTs } from './viteConfigTs';  

export default ({happName}: {happName: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  '.gitignore': gitignore(),
  'README.md': readmeMd(),
  'index.html': indexHtml(),
  'package.json': packageJson({happName}),
  'public': $public$(),
  'src': src({happName}),
  'tsconfig.json': tsconfigJson(),
  'vite.config.ts': viteConfigTs()
  }
})