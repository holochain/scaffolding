import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const moduleNamePluralMockJs = ({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleName}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; moduleName: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `import { hashToString } from 'holochain-ui-test-utils';

export class ${moduleNamePluralTitleCase}Mock {
  constructor() {
    this.agents = [];
  }

  create${moduleNameSnakeCase}({ username }, provenance) {
    const agent = {
      agent_pub_key: hashToString(provenance),
      ${moduleName}: { username, fields: {} },
    };
    this.agents.push(agent);

    return agent;
  }

  search${moduleNameSnakeCase}s({ username_prefix }) {
    return this.agents
      .filter(a => a.${moduleName}.username.startsWith(username_prefix.slice(0, 3)))
      .map(a => ({
        agent_pub_key: a.agent_pub_key,
        ...a,
      }));
  }

  get_my${moduleNameSnakeCase}(_, provenance) {
    const agent = this.findAgent(hashToString(provenance));

    if (!agent)
      return {
        agent_pub_key: hashToString(provenance),
      };
    return {
      agent_pub_key: agent.agent_pub_key,
      ${moduleName}: agent ? agent.${moduleName} : undefined,
    };
  }

  get_agent${moduleNameSnakeCase}({ agent_address }) {
    const agent = this.findAgent(agent_address);
    return agent ? agent.username : undefined;
  }

  findAgent(agent_address) {
    return this.agents.find(user => user.agent_pub_key === agent_address);
  }
}
`
});
    