import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const typesTs = ({moduleNamePluralTitleCase, moduleNameTitleCase, moduleName}: {moduleNamePluralTitleCase: string; moduleNameTitleCase: string; moduleName: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `import { AgentPubKeyB64, Dictionary } from '@holochain-open-dev/core-types';

export interface ${moduleNameTitleCase} {
  nickname: string;
  fields: Dictionary<string>;
}

export interface Agent${moduleNameTitleCase} {
  agentPubKey: AgentPubKeyB64;
  ${camelCase(moduleName)}: ${moduleNameTitleCase};
}

export interface Search${moduleNamePluralTitleCase}Input {
  nicknamePrefix: string;
}`
});
    