import { PatcherFile, PatcherNodeType } from '@patcher/types';

export const npmRc = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `engine-strict=true
`,
});
