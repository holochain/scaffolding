import { ScFile, ScNodeType } from '@source-craft/types';

export const npmRc = (): ScFile => ({
  type: ScNodeType.File,
  content: `engine-strict=true
`,
});
