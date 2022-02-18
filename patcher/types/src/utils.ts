import { PatcherDirectory, PatcherNode, PatcherNodeType } from './types';

export function findByPath(dir: PatcherDirectory, path: string): PatcherNode | undefined {
  const components = path.split('/');

  let currentDir: PatcherNode = dir;

  for (const component of components) {
    if (!currentDir || currentDir.type === PatcherNodeType.File) return undefined;
    currentDir = currentDir.children[component];
  }

  return currentDir;
}
