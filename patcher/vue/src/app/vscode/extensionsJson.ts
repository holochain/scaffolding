import { PatcherFile, PatcherNodeType } from '@patcher/types'; 

export const extensionsJson = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `{
  "recommendations": ["johnsoncodehk.volar"]
}
`
});
    