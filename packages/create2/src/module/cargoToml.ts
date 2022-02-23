import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const cargoToml = ({cargoThingDev, cargoThingRelease}: {cargoThingDev: string; cargoThingRelease: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `[workspace]
members = [
  "crates/zome",
  "crates/types",
]

${cargoThingDev}
opt-level = "z"

${cargoThingRelease}
opt-level = "z"
`
});
    