import { ScFile, ScNodeType } from '@source-craft/types';

export function addNpmDependency(packageJsonFile: ScFile, packageName: string, version: string): ScFile {
  const packageJson = JSON.parse(packageJsonFile.content);
  packageJson.dependencies[packageName] = version;

  return {
    type: ScNodeType.File,
    content: JSON.stringify(packageJson, null, 2),
  };
}
