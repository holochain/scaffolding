import { HappDefinition } from '@holochain/rad-definitions';
import { PatcherFile, PatcherNodeType } from '@patcher/types';
import { mergeStrings } from '../utils';

export const workspaceCargoToml = (happ: HappDefinition): PatcherFile => ({
  type: PatcherNodeType.File,
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
