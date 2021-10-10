//@ts-ignore
import semver from 'semver';
import chalk from 'chalk';
import { launchApp } from './app';
import clearCache from 'clear-npx-cache';

console.log('@holochain/create v0.0.18');

//do something when app is closing
process.on('exit', () => clearCache());

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
