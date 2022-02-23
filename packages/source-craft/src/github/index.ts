import { testWorkflow } from './test';
import { PatcherDirectory, PatcherNodeType } from '@source-craft/types';

export function githubWorkfows(): PatcherDirectory {
  return {
    type: PatcherNodeType.Directory,
    children: {
      workflows: {
        type: PatcherNodeType.Directory,
        children: {
          'test.yml': testWorkflow(),
        },
      },
    },
  };
}
