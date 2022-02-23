import { ScDirectory, ScNode, ScNodeType } from './types';

export function findByPath(dir: ScDirectory, path: string): ScNode | undefined {
  const components = path.split('/');

  let currentDir: ScNode = dir;

  for (const component of components) {
    if (!currentDir || currentDir.type === ScNodeType.File) return undefined;
    currentDir = currentDir.children[component];
  }

  return currentDir;
}
