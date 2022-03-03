import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { gitignore } from './gitignore';
import { indexHtml } from './indexHtml';
import { packageJson } from './packageJson';
import { rollupConfigJs } from './rollupConfigJs';
import src from './src';
import { tsconfigJson } from './tsconfigJson';
import { webDevServerConfigMjs } from './webDevServerConfigMjs';  

export default ({happName}: {happName: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  '.gitignore': gitignore(),
  'index.html': indexHtml(),
  'package.json': packageJson(),
  'rollup.config.js': rollupConfigJs(),
  'src': src({happName}),
  'tsconfig.json': tsconfigJson(),
  'web-dev-server.config.mjs': webDevServerConfigMjs()
  }
})