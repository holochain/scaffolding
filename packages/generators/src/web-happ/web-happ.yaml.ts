export default ({
  happName,
  uiBundlePath,
  happBundlePath,
}: {
  happName: string;
  uiBundlePath: string;
  happBundlePath: string;
}) =>
  `---
manifest_version: "1"
name: ${happName}
ui:
  bundled: "${uiBundlePath}"
happ_manifest:
  bundled: "${happBundlePath}"
`;
