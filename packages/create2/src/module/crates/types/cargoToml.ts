import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const cargoToml = ({moduleNameSnakeCase, moduleNamePlural}: {moduleNameSnakeCase: string; moduleNamePlural: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `[package]
authors = ["guillem.cordoba@gmail.com", "eric@harris-braun.com", "tatsuya.g.sato@yumeville.com"]
description = "Types for the hc_zome${moduleNameSnakeCase}s zome"
documentation = "https://holochain-open-dev.github.io/${snakeCase(moduleNamePlural)}"
edition = "2018"
homepage = "https://docs.rs/hc_zome${moduleNameSnakeCase}s_types"
license = "MIT"
name = "hc_zome${moduleNameSnakeCase}s_types"
repository = "https://github.com/holochain-open-dev/${snakeCase(moduleNamePlural)}"
version = "0.0.1"

[lib]
crate-type = ["cdylib", "rlib"]
name = "hc_zome${moduleNameSnakeCase}s_types"

[dependencies]
derive_more = "0"
serde = "1"

hdk = {version = "0.0.121", features = ["encoding"]}
`
});
    