import { ScFile, ScNodeType } from '@source-craft/types';

export const integrityZomeCargoToml = (integrityZomeName: string, author: string, hdiVersion: string): ScFile => ({
  type: ScNodeType.File,
  content: `[package]
edition = "2021"
name = "${integrityZomeName}_integrity"
version = "0.0.1"

[lib]
crate-type = ["cdylib", "rlib"]
name = "${integrityZomeName}_integrity"

[dependencies]
serde = "1"
derive_more = "0"

holochain_deterministic_integrity = "${hdiVersion}"
`,
});
