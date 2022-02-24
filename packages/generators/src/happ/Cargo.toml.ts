import { HappDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { mergeStrings } from '../utils';

export const workspaceCargoToml = (happ: HappDefinition): ScFile => ({
  type: ScNodeType.File,
  content: `[workspace]
resolver = "2"
members = [
${mergeStrings(
  happ.dnas.map(dna =>
    dna.zomes.map(
      zome => `    "${happ.dnas.length > 1 ? `dnas/${dna.name}` : 'dna'}/zomes/${zome.name}",
`,
    ),
  ),
)}]

[profile.dev]
opt-level = "z"

[profile.release]
opt-level = "z"
`,
});
