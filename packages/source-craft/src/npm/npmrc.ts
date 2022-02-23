import { PatcherFile, PatcherNodeType } from '@source-craft/types';

export const npmRc = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `engine-strict=true
`,
});
