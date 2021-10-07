import { FileChanges, HappDefinition, FileChangesType } from '../../types';

import packageJson from './package.json';
import npmrc from './npmrc';

export function generateRootPackageJson(happ: HappDefinition): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'package.json',
      content: packageJson(happ),
    },
    {
      type: FileChangesType.Create,
      fileName: '.npmrc',
      content: npmrc(),
    },
  ];
}
