import { ScFile, ScNodeType } from '@source-craft/types';

export const gitignore = (): ScFile => ({
  type: ScNodeType.File,
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
