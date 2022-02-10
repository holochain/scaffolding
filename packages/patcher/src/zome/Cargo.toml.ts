import { PatcherFile, PatcherNodeType } from '@patcher/types';

export const zomeCargoToml = (zomeName: string, author: string, hdkVersion = '0.0.120'): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `[package]
edition = "2018"
name = "${zomeName}"
version = "0.0.1"

[lib]
crate-type = ["cdylib", "rlib"]
name = "${zomeName}"

[dependencies]
serde = "1"
derive_more = "0"

hdk = {version="${hdkVersion}", features = ["encoding"]}
`,
});
