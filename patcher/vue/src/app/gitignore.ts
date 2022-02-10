import { PatcherFile, PatcherNodeType } from '@patcher/types'; 

export const gitignore = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `node_modules
.DS_Store
dist
dist-ssr
*.local
`
});
    