import { ScFile, ScNodeType } from '@source-craft/types';

export const integrityZomeCargoToml = (
  integrityZomeName: string,
  author: string,
  hdkVersion: string,
  hdiVersion: string,
): ScFile => ({
  type: ScNodeType.File,
  content: `[package]
edition = "2021"
name = "${integrityZomeName}"
version = "0.0.1"

[lib]
crate-type = ["cdylib", "rlib"]
name = "${integrityZomeName}"

[dependencies]
serde = "1"
derive_more = "0"

holochain_deterministic_integrity = "${hdiVersion}"
`,
});
