import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const gitignore = (): PatcherFile => ({
  type: PatcherNodeType.File,
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
/_site-dev/
/dist/
/bundle/
/out-tsc/
/storybook-static/
.hc*

## Rocket ignore files (need to be the full relative path to the folders)
.hc
*.tsbuildinfo
`
});
    