//@ts-ignore
import semver from 'semver';
import chalk from 'chalk';
import { launchApp } from './app';

const pkg = require('../package.json');

console.log(`@holochain/create v${pkg.version}`);

(async () => {
  try {
    if (!semver.gte(process.version, '14.0.0')) {
      console.log(chalk.bgRed('\nUh oh! Looks like you dont have Node v14 installed!\n'));
      console.log(`You can do this by going to ${chalk.underline.blue(`https://nodejs.org/`)}
  Or if you use nvm:
    $ nvm install node ${chalk.gray(`# "node" is an alias for the latest version`)}
    $ nvm use node
  `);
    } else {
      launchApp();
    }
  } catch (err) {
    console.log(err);
  }
})();
