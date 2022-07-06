import { ScFile, ScNodeType } from '@source-craft/types';

export const coordinatorZomeCargoToml = (coordinatorZomeName: string, author: string, hdkVersion: string): ScFile => ({
  type: ScNodeType.File,
  content: `[package]
edition = "2021"
name = "${coordinatorZomeName}"
version = "0.0.1"

[lib]
crate-type = ["cdylib", "rlib"]
name = "${coordinatorZomeName}"

[dependencies]
serde = "1"
derive_more = "0"
${coordinatorZomeName}_integrity = { path = "../integrity" }

hdk = {version="${hdkVersion}", features = ["encoding"]}
`,
});
