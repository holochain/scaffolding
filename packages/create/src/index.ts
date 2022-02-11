//@ts-ignore
import semver from 'semver';
import chalk from 'chalk';
import { launchApp } from './app';
import { init } from './init';

const pkg = require('../package.json');

console.log(`@holochain/create v${pkg.version}`);

try {
  if (!semver.gte(process.version, '14.0.0')) {
    console.log(chalk.bgRed('\nUh oh! Looks like you dont have Node v14 installed!\n'));
    console.log(`You can do this by going to ${chalk.underline.blue(`https://nodejs.org/`)}
  Or if you use nvm:
    $ nvm install node ${chalk.gray(`# "node" is an alias for the latest version`)}
    $ nvm use node
  `);
  } else {
    if (process.argv.length > 2 && process.argv[2] === 'init') {
      const appName = process.argv[3] || 'my-app';

      init(appName);
    } else {
      launchApp();
    }
  }
} catch (err) {
  console.log(err);
}
