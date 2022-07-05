import { ScFile, ScNodeType } from '@source-craft/types';

export const zomeCargoToml = (zomeName: string, author: string, hdkVersion: string, hdiVersion: string): ScFile => ({
  type: ScNodeType.File,
  content: `[package]
edition = "2021"
name = "${zomeName}"
version = "0.0.1"

[lib]
crate-type = ["cdylib", "rlib"]
name = "${zomeName}"

[dependencies]
serde = "1"
derive_more = "0"

hdk = {version="${hdkVersion}", features = ["encoding"]}
holochain_deterministic_integrity = "${hdiVersion}"
`,
});
