import { HappDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';

export const workspaceCargoToml = (happ: HappDefinition): ScFile => ({
  type: ScNodeType.File,
  content: `[workspace]
resolver = "2"
members = [
  "${happ.dnas.length > 1 ? `dnas/*` : 'dna'}/integrity_zomes/*",
  "${happ.dnas.length > 1 ? `dnas/*` : 'dna'}/coordinator_zomes/*"
]

[profile.dev]
opt-level = "z"

[profile.release]
opt-level = "z"
`,
});
