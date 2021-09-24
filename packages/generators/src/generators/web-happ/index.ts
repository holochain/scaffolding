import { generateHapp } from '../happ';
import { FileChanges, FileChangesType, HappDefinition } from '../../types';

import webHappYaml from './web-happ.yaml';

export function generateWebHappYaml({
  happName,
  uiBundlePath,
  happBundlePath,
}: {
  happName: string;
  uiBundlePath: string;
  happBundlePath: string;
}): FileChanges[] {
  return [
    {
      type: FileChangesType.Create,
      fileName: 'web-happ.yaml',
      content: webHappYaml({
        happName,
        uiBundlePath,
        happBundlePath,
      }),
    },
  ];
}

export function generateWebHapp(happ: HappDefinition): FileChanges[] {
  return [
    ...generateHapp(happ),
    {
      type: FileChangesType.InDir,
      dirName: 'workdir',
      changes: [
        ...generateWebHappYaml({
          happName: happ.name,
          uiBundlePath: '../ui/dist.zip',
          happBundlePath: `./${happ.name}.happ`,
        }),
      ],
    },
  ];
}
