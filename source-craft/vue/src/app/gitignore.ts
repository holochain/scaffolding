import { ScFile, ScNodeType } from '@source-craft/types'; 

export const gitignore = (): ScFile => ({
  type: ScNodeType.File,
  content: `node_modules
.DS_Store
dist
dist-ssr
*.local
`
});
    