import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const happYaml = ({kebabPlural_, moduleNamePlural}: {kebabPlural_: string; moduleNamePlural: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `---
manifest_version: "1"
name: ${kebabPlural_}test
description: ~
roles:
  - id: ${snakeCase(moduleNamePlural)}
    provisioning:
      strategy: create
      deferred: false
    dna:
      bundled: "../dna/${kebabPlural_}test.dna"
      properties: ~
      uuid: ~
      version: ~
      clone_limit: 0
`
});
    