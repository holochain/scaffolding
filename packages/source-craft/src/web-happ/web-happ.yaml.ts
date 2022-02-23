import { PatcherFile, PatcherNodeType } from '@source-craft/types';

export const webHappYaml = ({
  happName,
  uiBundlePath,
  happBundlePath,
}: {
  happName: string;
  uiBundlePath: string;
  happBundlePath: string;
}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `---
manifest_version: "1"
name: ${happName}
ui:
  bundled: "${uiBundlePath}"
happ_manifest:
  bundled: "${happBundlePath}"
`,
});
