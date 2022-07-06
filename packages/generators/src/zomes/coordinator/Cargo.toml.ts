import { ZomeBundleDefinition } from '@holochain-scaffolding/definitions';
import { ScFile, ScNodeType } from '@source-craft/types';
import { getCoordinatorCrateName } from '../utils';

export const coordinatorZomeCargoToml = (coordinatorCrateName: string, integrityCrateName: string, author: string, hdkVersion: string): ScFile => ({
  type: ScNodeType.File,
  content: `[package]
edition = "2021"
name = "${coordinatorCrateName}"
version = "0.0.1"

[lib]
crate-type = ["cdylib", "rlib"]
name = "${coordinatorCrateName}"

[dependencies]
serde = "1"
derive_more = "0"
${integrityCrateName} = { path = "../integrity" }

hdk = {version="${hdkVersion}", features = ["encoding"]}
`,
});
