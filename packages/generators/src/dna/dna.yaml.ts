import { IntegrityZomeDefinition, CoordinatorZomeDefinition, HappDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { mergeStrings } from '../utils';
import { getCoordinatorCrateName, getIntegrityCrateName } from '../zomes';

export const dnaYaml = (happ: HappDefinition, dnaIndex: number, pathToBase: string): ScFile => {
  const dna = happ.dnas[dnaIndex];
  return {
    type: ScNodeType.File,
    content: `---
manifest_version: "1"
name: ${dna.name}
integrity:
  uuid: 00000000-0000-0000-0000-000000000000
  properties: ~
  origin_time: ${new Date().toISOString()}
  zomes:
${mergeStrings(
dna.integrityZomes.map(
  (zome: IntegrityZomeDefinition, zomeIndex: number) =>
`    - name: ${zome.name}
      bundled: ${pathToBase}target/wasm32-unknown-unknown/release/${getIntegrityCrateName(happ, dnaIndex, zomeIndex)}.wasm
`,
  ),
)}
coordinator:
  zomes:
${mergeStrings(
  dna.coordinatorZomes.map(
    (zome: CoordinatorZomeDefinition, zomeIndex: number) =>
  `    - name: ${zome.name}
        bundled: ${pathToBase}target/wasm32-unknown-unknown/release/${getCoordinatorCrateName(happ, dnaIndex, zomeIndex)}.wasm
  `,
  ),
)}
`,
  };
};
