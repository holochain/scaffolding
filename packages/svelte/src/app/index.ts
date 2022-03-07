import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { gitignore } from './gitignore';
import { packageJson } from './packageJson';
import $public$ from './$public$';
import { rollupConfigJs } from './rollupConfigJs';
import src from './src';
import { tsconfigJson } from './tsconfigJson';
import { webDevServerConfigMjs } from './webDevServerConfigMjs';  

export default ({happName, subcomponentImports, appContent}: {happName: string; subcomponentImports: string; appContent: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  '.gitignore': gitignore(),
  'package.json': packageJson(),
  'public': $public$(),
  'rollup.config.js': rollupConfigJs(),
  'src': src({happName, subcomponentImports, appContent}),
  'tsconfig.json': tsconfigJson(),
  'web-dev-server.config.mjs': webDevServerConfigMjs()
  }
})