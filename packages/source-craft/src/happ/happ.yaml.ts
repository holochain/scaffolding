import { HappDefinition } from '@holochain-scaffolding/definitions';
import { PatcherFile, PatcherNodeType } from '@source-craft/types';
import { getDnaBundlePath, mergeStrings } from '../utils';

export const happYaml = (happ: HappDefinition): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `---
manifest_version: "1"
name: ${happ.name}
description: ~
roles:
${mergeStrings(
  happ.dnas.map(
    dna => `
  - id: ${dna.name}
    provisioning:
      strategy: create
      deferred: false
    dna:
      bundled: "../${getDnaBundlePath(happ, dna.name)}"
      properties: ~
      uuid: ~
      version: ~
      clone_limit: 0
`,
  ),
)}
`,
});
