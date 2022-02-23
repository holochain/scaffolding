import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const readmeMd = ({moduleNamePluralTitleCase, moduleNamePlural}: {moduleNamePluralTitleCase: string; moduleNamePlural: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `# ${moduleNamePluralTitleCase} Module

Small zome to manage the ${moduleNamePlural} in your DNA, in RSM.

This module is designed to be included in other DNAs, assuming as little as possible from those. It is packaged as a holochain zome, and an npm package that offers native Web Components that can be used across browsers and frameworks.

> Notice that this zome still stores all ${moduleNamePlural} in the DNA in which the zome is included. Integration and bridging with personas & ${moduleNamePlural} will be done in the future, maintaining as much as possible the current API.

## Documentation

See our [installation instructions and documentation](https://holochain-open-dev.github.io/${moduleNamePlural}).

## Developer setup

Visit the [developer setup](/dev-setup.md).
`
});
    