import { FileChanges, HappDefinition, FileChangesType } from '../../types';

import packageJson from './package.json';

export function generateRootPackageJson(happ: HappDefinition): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'package.json',
      content: packageJson(happ),
    },
  ];
}
