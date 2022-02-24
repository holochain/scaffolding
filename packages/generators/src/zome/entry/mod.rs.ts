import { ScFile, ScNodeType } from '@source-craft/types';

export const modRs = (): ScFile => ({
  type: ScNodeType.File,
  content: `mod handlers;
mod entry;

pub use handlers::*;
pub use entry::*;
`,
});
