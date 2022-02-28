import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { gitignore } from './gitignore';
import { readmeMd } from './readmeMd';
import { indexHtml } from './indexHtml';
import { packageJson } from './packageJson';
import $public$ from './$public$';
import src from './src';
import { tsconfigJson } from './tsconfigJson';
import { viteConfigTs } from './viteConfigTs';  

export default ({happName, appContent, appSubcomponents}: {happName: string; appContent: string; appSubcomponents: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  '.gitignore': gitignore(),
  'README.md': readmeMd(),
  'index.html': indexHtml(),
  'package.json': packageJson({happName}),
  'public': $public$(),
  'src': src({happName, appContent, appSubcomponents}),
  'tsconfig.json': tsconfigJson(),
  'vite.config.ts': viteConfigTs()
  }
})