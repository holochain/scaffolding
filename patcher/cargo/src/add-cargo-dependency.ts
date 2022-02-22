import { PatcherFile, PatcherNodeType } from '@patcher/types';

export function addCargoDependency(packageJsonFile: PatcherFile, packageName: string, version: string): PatcherFile {
  const packageJson = JSON.parse(packageJsonFile.content);
  packageJson.dependencies[packageName] = version;

  return {
    type: PatcherNodeType.File,
    content: JSON.stringify(packageJson, null, 2),
  };
}
