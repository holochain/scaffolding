import { PatcherFile, PatcherNodeType } from '@source-craft/types';

export const gitignore = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `node_modules/
/dist/
/target/
/.cargo/
*.happ
*.webhapp
*.zip
*.dna
.hc*
.hc
.running
`,
});
