import { ZomeBundleDefinition, HappDefinition } from '@holochain-scaffolding/definitions';
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
dna.zomeBundles.map(
  (zomeBundle: ZomeBundleDefinition, zomeBundleIndex: number) =>
`    - name: ${zomeBundle.integrityZome.name}
      bundled: ${pathToBase}target/wasm32-unknown-unknown/release/${getIntegrityCrateName(happ, dnaIndex, zomeBundleIndex)}.wasm
`,
  ),
)}
coordinator:
  zomes:
${mergeStrings(
dna.zomeBundles.map(
  (zomeBundle: ZomeBundleDefinition, zomeIndex: number) =>
`    - name: ${zomeBundle.coordinatorZome.name}
      bundled: ${pathToBase}target/wasm32-unknown-unknown/release/${getCoordinatorCrateName(happ, dnaIndex, zomeIndex)}.wasm
      dependencies:
        - name: ${zomeBundle.integrityZome.name}
`,
),
)}
`,
  };
};
