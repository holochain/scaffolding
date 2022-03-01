import fs from 'fs';
import { readFolder, directoryToGenerator, Case, writeDirectoryTree } from '@source-craft/fs';
import { fileURLToPath } from 'url';
import path from 'path';
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const d = readFolder(`${__dirname}/template`);

const caseByExtensions = {
  rs: Case.SnakeCase,
  ts: Case.CamelCase,
  toml: Case.SnakeCase,
  yaml: Case.SnakeCase,
};
const caseByExtensionsKebab = {
  rs: Case.KebabCase,
  ts: Case.KebabCase,
  toml: Case.KebabCase,
};

const patched = directoryToGenerator(d, [
  { literal: '[profile.dev]', template: 'cargoThingDev' },
  { literal: '[profile.release]', template: 'cargoThingRelease' },
  { literal: '@holochain-open-dev/profiles', template: 'packageName' },
  { literal: `const zomeName = 'profiles';`, template: 'testZomeName' },
  {
    literal: '_profile',
    template: 'moduleNameSnakeCase',
  },
  {
    literal: 'Profiles',
    template: 'moduleNamePluralTitleCase',
  },
  {
    literal: '-profile',
    template: '_kebab',
  },
  {
    literal: 'profiles-',
    template: 'kebabPlural_',
  },
  {
    literal: 'profile-',
    template: 'kebabSingular_',
  },
  {
    literal: 'Profile',
    template: 'moduleNameTitleCase',
  },
  {
    literal: 'profiles',
    template: 'moduleNamePlural',
    caseByExtensions,
  },
  {
    literal: 'profile',
    template: 'moduleName',
    caseByExtensions,
  },
]);

if (!fs.existsSync('./src')) fs.mkdirSync('./src');

writeDirectoryTree(`${__dirname}/src/module`, patched);
