import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const addingTheZomeMd = ({moduleNameSnakeCase, moduleNamePlural}: {moduleNameSnakeCase: string; moduleNamePlural: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `# Setting Up >> Adding the Zome ||10

1. In your \`zomes\` folder, run \`cargo new ${moduleNamePlural} --lib\`.
2. Add this zome as a dependency in the \`Cargo.toml\` file:

\`\`\`toml
[dependencies]
hc_zome${moduleNameSnakeCase}s = {git = "https://github.com/holochain-open-dev/${moduleNamePlural}", rev = "for-hc-v0.0.124", package = "hc_zome${moduleNameSnakeCase}s"}
\`\`\`

Replace the \`rev\` field with the holochain version you are using. See [which tags are available](https://github.com/holochain-open-dev/${moduleNamePlural}/tags).

3.  Replace the contents of the \`lib.rs\` with this content:

\`\`\`rust
extern crate hc_zome${moduleNameSnakeCase}s;
\`\`\`

4. Add this new crate to your top level \`Cargo.toml\`.
5. Add the zome into your \`dna.yaml\` file.
6. Compile the DNA with the usual \`CARGO_TARGET_DIR=target cargo build --release --target wasm32-unknown-unknown\`.
`
});
    