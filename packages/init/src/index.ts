import 'babel-polyfill';
//@ts-ignore
import semver from 'semver';
import chalk from 'chalk';

import { webHapp } from '@holochain/rad-patcher';
import { generateVueApp } from '@patcher/vue';
import { applyPatch } from '@patcher/fs';

const pkg = require('../package.json');

console.log(`@holochain/init v${pkg.version}`);

try {
  if (!semver.gte(process.version, '14.0.0')) {
    console.log(chalk.bgRed('\nUh oh! Looks like you dont have Node v14 installed!\n'));
    console.log(`You can do this by going to ${chalk.underline.blue(`https://nodejs.org/`)}
  Or if you use nvm:
    $ nvm install node ${chalk.gray(`# "node" is an alias for the latest version`)}
    $ nvm use node
  `);
  }
} catch (err) {
  console.log(err);
}

const appName = process.argv[2] || 'my-app';

webHapp(
  {
    name: appName,
    dnas: [
      {
        name: 'dna-1',
        zomes: [
          {
            entry_defs: [
              {
                create: true,
                delete: true,
                update: true,
                read: true,
                sample: {
                  foo: 1,
                  bar: 'some content',
                },
                name: 'entry-def-1',
              },
            ],
            name: 'zome-1',
          },
        ],
      },
    ],
  },
  generateVueApp(),
).then(d => {
  applyPatch(`${process.cwd()}/${appName}`, d);
});
