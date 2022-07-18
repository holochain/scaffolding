import { HappDefinition, IntegrityZomeDefinition, CoordinatorZomeDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { mergeStrings } from '../utils';
import { getCoordinatorCrateName, getIntegrityCrateName } from '../zomes/utils';

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
  dna.integrity_zomes.map(
    (integrityZome: IntegrityZomeDefinition, integrityZomeIndex: number) =>
      `    - name: ${integrityZome.name}
      bundled: ${pathToBase}target/wasm32-unknown-unknown/release/${getIntegrityCrateName(
        happ,
        dnaIndex,
        integrityZomeIndex,
      )}.wasm
`,
  ),
)}
coordinator:
  zomes:
${mergeStrings(
  dna.coordinator_zomes.map(
    (coordinatorZome: CoordinatorZomeDefinition, zomeIndex: number) =>
      `    - name: ${coordinatorZome.name}
      bundled: ${pathToBase}target/wasm32-unknown-unknown/release/${getCoordinatorCrateName(
        happ,
        dnaIndex,
        zomeIndex,
      )}.wasm
      dependencies:${coordinatorZome.dependencies.map(
        d => `
        - name: ${d}`,
      )}
`,
  ),
)}
`,
  };
};
