// TODO: add author
export default ({ zomeName, author, hdkVersion }: { zomeName: string; hdkVersion: string; author: string }) =>
  `[package]
edition = "2018"
name = "${zomeName}"
version = "0.0.1"

[lib]
crate-type = ["cdylib", "rlib"]
name = "${zomeName}"

[dependencies]
serde = "1"
derive_more = "0"
chrono = {version = "0.4", features = ["serde"]}

hdk = {version="${hdkVersion}", features = ["enconding"]}
`;
