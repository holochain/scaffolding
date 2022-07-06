import { ScFile, ScNodeType } from '@source-craft/types';

export const modRs = (): ScFile => ({
  type: ScNodeType.File,
  content: `mod definition;
pub use definition::*;
`,
});
