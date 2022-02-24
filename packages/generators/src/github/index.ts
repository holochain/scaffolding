import { testWorkflow } from './test';
import { ScDirectory, ScNodeType } from '@source-craft/types';

export function githubWorkfows(): ScDirectory {
  return {
    type: ScNodeType.Directory,
    children: {
      workflows: {
        type: ScNodeType.Directory,
        children: {
          'test.yml': testWorkflow(),
        },
      },
    },
  };
}
