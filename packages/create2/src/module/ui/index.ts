import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { editorconfig } from './editorconfig';
import { eslintrcCjs } from './eslintrcCjs';
import { gitignore } from './gitignore';
import { readmeMd } from './readmeMd';
import { customElementsJson } from './customElementsJson';
import demo from './demo';
import { packageJson } from './packageJson';
import src from './src';
import test from './test';
import { tsconfigJson } from './tsconfigJson';
import { webDevServerConfigMjs } from './webDevServerConfigMjs';
import { webDevPluginsMjs } from './webDevPluginsMjs';
import { webTestRunnerConfigMjs } from './webTestRunnerConfigMjs';  

export default ({packageName, moduleNameSnakeCase, moduleNamePlural, moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleName}: {packageName: string; moduleNameSnakeCase: string; moduleNamePlural: string; moduleNamePluralTitleCase: string; _kebab: string; kebabPlural_: string; kebabSingular_: string; moduleNameTitleCase: string; moduleName: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  '.editorconfig': editorconfig(),
  '.eslintrc.cjs': eslintrcCjs(),
  '.gitignore': gitignore(),
  'README.md': readmeMd({packageName, moduleNameSnakeCase, moduleNamePlural}),
  'custom-elements.json': customElementsJson({moduleNameSnakeCase, moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName}),
  'demo': demo({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural}),
  'package.json': packageJson({packageName, moduleNameSnakeCase, _kebab, kebabSingular_}),
  'src': src({moduleNamePluralTitleCase, moduleNamePlural, moduleNameSnakeCase, kebabPlural_, _kebab, moduleNameTitleCase, kebabSingular_, moduleName}),
  'test': test({_kebab, moduleNameTitleCase, moduleName, moduleNamePluralTitleCase, moduleNamePlural, moduleNameSnakeCase}),
  'tsconfig.json': tsconfigJson(),
  'web-dev-server.config.mjs': webDevServerConfigMjs(),
  'web-dev.plugins.mjs': webDevPluginsMjs(),
  'web-test-runner.config.mjs': webTestRunnerConfigMjs()
  }
})