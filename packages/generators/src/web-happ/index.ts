import { HappDefinition } from '@holochain/rad-definitions';

import { generateHapp } from '../happ';
import { FileChanges, FileChangesType, InDir } from '../file-changes';

import webHappYaml from './web-happ.yaml';
import { generateTsTypes } from '../ts';

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

export async function generateWebHapp(happ: HappDefinition, uiFileChanges: FileChanges[]): Promise<FileChanges[]> {
  const src = uiFileChanges.find(change => change.type === FileChangesType.InDir && change.dirName === 'src') as InDir;

  src.changes = [...src.changes, ...(await generateTsTypes(happ))];

  return [
    ...(await generateHapp(happ)),
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
    {
      type: FileChangesType.InDir,
      dirName: 'ui',
      changes: uiFileChanges,
    },
  ];
}
