import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { gitignore } from './gitignore';
import vscode from './vscode';
import { readmeMd } from './readmeMd';
import { indexHtml } from './indexHtml';
import { packageJson } from './packageJson';
import $public$ from './public';
import src from './src';
import { tsconfigJson } from './tsconfigJson';
import { viteConfigTs } from './viteConfigTs';  

export default (): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  '.gitignore': gitignore(),
  '.vscode': vscode(),
  'README.md': readmeMd(),
  'index.html': indexHtml(),
  'package.json': packageJson(),
  'public': $public$(),
  'src': src(),
  'tsconfig.json': tsconfigJson(),
  'vite.config.ts': viteConfigTs()
  }
})