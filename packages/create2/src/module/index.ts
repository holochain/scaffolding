import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import github from './github';
import { gitignore } from './gitignore';
import { huskyrc } from './huskyrc';
import { cargoLock } from './cargoLock';
import { cargoToml } from './cargoToml';
import { license } from './license';
import { readmeMd } from './readmeMd';
import crates from './crates';
import { defaultNix } from './defaultNix';
import { devSetupMd } from './devSetupMd';
import docs from './docs';
import { packageLockJson } from './packageLockJson';
import { packageJson } from './packageJson';
import { rocketConfigJs } from './rocketConfigJs';
import tests from './tests';
import ui from './ui';
import workdir from './workdir';  

export default ({moduleNameSnakeCase, cargoThingDev, cargoThingRelease, moduleNamePluralTitleCase, moduleNamePlural, moduleNameTitleCase, moduleName, kebabPlural_, _kebab, kebabSingular_, packageName, testZomeName}: {moduleNameSnakeCase: string; cargoThingDev: string; cargoThingRelease: string; moduleNamePluralTitleCase: string; moduleNamePlural: string; moduleNameTitleCase: string; moduleName: string; kebabPlural_: string; _kebab: string; kebabSingular_: string; packageName: string; testZomeName: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  '.github': github(),
  '.gitignore': gitignore(),
  '.huskyrc': huskyrc(),
  'Cargo.lock': cargoLock({moduleNameSnakeCase}),
  'Cargo.toml': cargoToml({cargoThingDev, cargoThingRelease}),
  'LICENSE': license(),
  'README.md': readmeMd({moduleNamePluralTitleCase, moduleNamePlural}),
  'crates': crates({moduleNameSnakeCase, moduleNamePlural, moduleNamePluralTitleCase, moduleNameTitleCase, moduleName}),
  'default.nix': defaultNix(),
  'dev-setup.md': devSetupMd({kebabPlural_}),
  'docs': docs({moduleNameSnakeCase, moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName, packageName}),
  'package-lock.json': packageLockJson({packageName}),
  'package.json': packageJson({packageName, kebabPlural_}),
  'rocket.config.js': rocketConfigJs({moduleNamePlural}),
  'tests': tests({packageName, testZomeName, moduleNameSnakeCase, moduleNamePluralTitleCase, kebabPlural_, moduleNameTitleCase, moduleNamePlural, moduleName}),
  'ui': ui({packageName, moduleNameSnakeCase, moduleNamePlural, moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleName}),
  'workdir': workdir({moduleNameSnakeCase, kebabPlural_, moduleNamePlural})
  }
})