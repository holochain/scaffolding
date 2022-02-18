import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const cargoToml = ({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNamePlural}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; moduleNamePlural: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `[package]
authors = ["guillem.cordoba@gmail.com", "eric@harris-braun.com", "tatsuya.g.sato@yumeville.com"]
description = "${moduleNamePluralTitleCase} zome for any Holochain app"
documentation = "https://holochain-open-dev.github.io/${snakeCase(moduleNamePlural)}"
edition = "2018"
homepage = "https://docs.rs/hc_zome${moduleNameSnakeCase}s"
license = "MIT"
name = "hc_zome${moduleNameSnakeCase}s"
repository = "https://github.com/holochain-open-dev/${snakeCase(moduleNamePlural)}"
version = "0.0.1"

[lib]
crate-type = ["cdylib", "rlib"]
name = "hc_zome${moduleNameSnakeCase}s"

[dependencies]
derive_more = "0"
serde = "1"

hc_zome${moduleNameSnakeCase}s_types = {path = "../types"}
hdk = {version = "0.0.121", features = ["encoding"]}
`
});
    