import { ScFile, ScNodeType } from '@source-craft/types';

export const webHappYaml = ({
  happName,
  uiBundlePath,
  happBundlePath,
}: {
  happName: string;
  uiBundlePath: string;
  happBundlePath: string;
}): ScFile => ({
  type: ScNodeType.File,
  content: `---
manifest_version: "1"
name: ${happName}
ui:
  bundled: "${uiBundlePath}"
happ_manifest:
  bundled: "${happBundlePath}"
`,
});
