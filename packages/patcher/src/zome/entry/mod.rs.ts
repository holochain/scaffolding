import { PatcherFile, PatcherNodeType } from '@patcher/types';

export const modRs = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `mod handlers;
mod entry;

pub use handlers::*;
pub use entry::*;
`,
});
