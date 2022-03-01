// eslint-disable-next-line @typescript-eslint/ban-ts-comment
//@ts-ignore
import semver from 'semver';
import chalk from 'chalk';
import { writeDirectoryTree } from '@source-craft/fs';
import generateModule from './module';
import upperFirst from 'lodash-es/upperFirst';
import camelCase from 'lodash-es/camelCase';
import snakeCase from 'lodash-es/snakeCase';
import kebabCase from 'lodash-es/kebabCase';
import { patchProfiles } from './patch-profiles';

console.log(`@holochain-open-dev/create`);

if (!semver.gte(process.version, '14.0.0')) {
  console.log(chalk.bgRed('\nUh oh! Looks like you dont have Node v14 installed!\n'));
  console.log(`You can do this by going to ${chalk.underline.blue(`https://nodejs.org/`)}
  Or if you use nvm:
    $ nvm install node ${chalk.gray(`# "node" is an alias for the latest version`)}
    $ nvm use node
  `);
  process.exit(1);
}

if (!process.argv[2] || !process.argv[3]) {
  console.log(
    'Please provide the [SINGULAR_ITEM] and the [PLURAL_ITEM] names in kebab-case for the module: npm init @holochain-open-dev resource-booking resource-bookings',
  );
  process.exit(1);
}

const moduleName = process.argv[2];
const pluralName = process.argv[3];

let d = generateModule({
  moduleNameSnakeCase: `_${snakeCase(moduleName)}`,
  cargoThingDev: '[profile.dev]',
  cargoThingRelease: '[profile.release]',
  packageName: `@holochain-open-dev/${kebabCase(pluralName)}`,
  testZomeName: `const zomeName = '${snakeCase(pluralName)}';`,

  _kebab: `-${kebabCase(moduleName)}`,
  kebabPlural_: `${kebabCase(pluralName)}-`,
  kebabSingular_: `${kebabCase(moduleName)}-`,
  moduleName: moduleName,
  moduleNamePlural: pluralName,
  moduleNamePluralTitleCase: upperFirst(camelCase(pluralName)),
  moduleNameTitleCase: upperFirst(camelCase(moduleName)),
});

d = patchProfiles(d, moduleName, pluralName);

writeDirectoryTree(`${process.cwd()}/${kebabCase(pluralName)}`, d);

console.log(`Module scaffolded!\n`);
console.log(`Run these commands to get started:\n\n`);

console.log(`cd ${kebabCase(pluralName)}`);
console.log(`nix-shell`);
console.log(`npm install`);
console.log(`npm test`);
console.log(`npm start`);
