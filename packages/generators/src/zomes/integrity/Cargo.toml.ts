import { ScFile, ScNodeType } from '@source-craft/types';

export const integrityZomeCargoToml = (
  integrityZomeName: string,
  author: string,
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
serde = "=1.0.136"
chrono = { version = "0.4.22", default-features = false, features = ["clock", "std", "oldtime", "serde"], optional = true }
derive_more = "0"

hdi = "${hdiVersion}"
`,
});
