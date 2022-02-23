import { ScDirectory, ScNodeType } from '@source-craft/types';
import { readdirSync, existsSync, readFileSync } from 'fs';
import ignore, { Ignore } from 'ignore';
import cloneDeep from 'lodash-es/cloneDeep';
import { isDirectory } from './utils';

export function readFolder(path: string, ignoreManager: Ignore = ignore()): ScDirectory {
  ignoreManager.add('.git');
  if (existsSync('.gitignore')) {
    ignoreManager.add(readFileSync('.gitignore').toString());
  }

  const paths = readdirSync(path);

  const filteredPaths = ignoreManager.filter(paths);

  const directory: ScDirectory = {
    type: ScNodeType.Directory,
    children: {},
  };

  for (const childPath of filteredPaths) {
    const fullPath = `${path}/${childPath}`;
    if (isDirectory(fullPath)) {
      const d = readFolder(fullPath, cloneDeep(ignoreManager));

      directory.children[childPath] = d;
    } else {
      const content = readFileSync(fullPath, 'utf8');

      directory.children[childPath] = {
        type: ScNodeType.File,
        content,
      };
    }
  }

  return directory;
}
