import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const gitignore = (): ScFile => ({
  type: ScNodeType.File,
  content: `## editors
/.idea
/.vscode

## system files
.DS_Store

## npm
/node_modules/
/npm-debug.log

## testing
/coverage/

## temp folders
/.tmp/

# build
/_site/
/dist/
/out-tsc/

storybook-static
.rollup.cache`
});
    