import { FileChanges, FileChangesType } from '../../types/file-changes';
import { HappDefinition } from '../../types/happ';
import release from './release';
import test from './test';

export function generateGithubWorkfows(happ: HappDefinition): FileChanges[] {
  return [
    {
      type: FileChangesType.InDir,
      dirName: '.github',
      changes: [
        {
          type: FileChangesType.InDir,
          dirName: 'workflows',
          changes: [
            {
              type: FileChangesType.Create,
              fileName: 'release.yml',
              content: release(happ.name),
            },
            {
              type: FileChangesType.Create,
              fileName: 'test.yml',
              content: test(),
            },
          ],
        },
      ],
    },
  ];
}
