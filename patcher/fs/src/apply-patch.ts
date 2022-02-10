import { PatcherDirectory, PatcherNodeType } from '@patcher/types';
import { readdirSync, writeFileSync, mkdirSync, existsSync, rmSync, rmdirSync } from 'fs';
import { isDirectory } from './utils';

export function applyPatch(sourcePath: string, targetFs: PatcherDirectory): void {
  if (!existsSync(sourcePath)) {
    mkdirSync(sourcePath);
  }

  const paths = readdirSync(sourcePath);

  // Create and overwrite dirs and files
  for (const [childPath, child] of Object.entries(targetFs.children)) {
    const fullChildPath = `${sourcePath}/${childPath}`;

    if (child.type === PatcherNodeType.Directory) {
      applyPatch(fullChildPath, child);
    } else {
      writeFileSync(fullChildPath, child.content);
    }
  }

  // Remove the existing files that are not in the children
  const pathsToRemove = paths.filter(path => !Object.keys(targetFs.children).includes(path));

  for (const pathToRemove of pathsToRemove) {
    if (isDirectory(pathToRemove)) {
      rmdirSync(pathToRemove);
    } else {
      rmSync(pathToRemove);
    }
  }
}
