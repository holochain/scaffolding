import { HappDefinition } from '@holochain/rad-definitions';
import { getDnaBundlePath, mergeStrings } from '../utils';

export default (happ: HappDefinition) =>
  `---
manifest_version: "1"
name: ${happ.name}
description: ~
roles:
${mergeStrings(happ.dnas.map(
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
))}
`;
