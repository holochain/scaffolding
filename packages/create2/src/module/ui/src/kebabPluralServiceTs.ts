import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const kebabPluralServiceTs = ({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase, moduleNamePlural, moduleName}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `import { CellClient } from '@holochain-open-dev/cell-client';
import { AgentPubKeyB64 } from '@holochain-open-dev/core-types';
import { Agent${moduleNameTitleCase}, ${moduleNameTitleCase} } from './types';

export class ${moduleNamePluralTitleCase}Service {
  constructor(public cellClient: CellClient, public zomeName = '${camelCase(moduleNamePlural)}') {}

  /**
   * Get my ${camelCase(moduleName)}, if it has been created
   * @returns my ${camelCase(moduleName)}
   */
  async getMy${moduleNameTitleCase}(): Promise<Agent${moduleNameTitleCase}> {
    return this.callZome('get_my${moduleNameSnakeCase}', null);
  }

  /**
   * Get the ${camelCase(moduleName)} for the given agent, if they have created it
   * 
   * @param agentPubKey the agent to get the ${camelCase(moduleName)} for
   * @returns the ${camelCase(moduleName)} of the agent
   */
  async getAgent${moduleNameTitleCase}(agentPubKey: AgentPubKeyB64): Promise<Agent${moduleNameTitleCase}> {
    return this.callZome('get_agent${moduleNameSnakeCase}', agentPubKey);
  }

  /**
   * Get the ${camelCase(moduleNamePlural)} for the given agent
   * 
   * @param agentPubKeys the agents to get the ${camelCase(moduleName)} for
   * @returns the ${camelCase(moduleName)} of the agents, in the same order as the input parameters
   */
  async getAgents${moduleNamePluralTitleCase}(
    agentPubKeys: AgentPubKeyB64[]
  ): Promise<Agent${moduleNameTitleCase}[]> {
    return this.callZome('get_agents${moduleNameSnakeCase}', agentPubKeys);
  }

  /**
   * Search ${camelCase(moduleNamePlural)} that start with nicknamePrefix
   * 
   * @param nicknamePrefix must be of at least 3 characters
   * @returns the ${camelCase(moduleNamePlural)} with the nickname starting with nicknamePrefix
   */
  async search${moduleNamePluralTitleCase}(nicknamePrefix: string): Promise<Array<Agent${moduleNameTitleCase}>> {
    return this.callZome('search${moduleNameSnakeCase}s', {
      nicknamePrefix: nicknamePrefix,
    });
  }

  /**
   * Get the ${camelCase(moduleNamePlural)} for all the agents in the DHT
   * 
   * @returns the ${camelCase(moduleNamePlural)} for all the agents in the DHT
   */
  async getAll${moduleNamePluralTitleCase}(): Promise<Array<Agent${moduleNameTitleCase}>> {
    return this.callZome('get_all${moduleNameSnakeCase}s', null);
  }

  /**
   * Create my ${camelCase(moduleName)}
   * 
   * @param ${camelCase(moduleName)} the ${camelCase(moduleName)} to create
   * @returns my ${camelCase(moduleName)} with my agentPubKey
   */
   async create${moduleNameTitleCase}(${camelCase(moduleName)}: ${moduleNameTitleCase}): Promise<Agent${moduleNameTitleCase}> {
    const ${camelCase(moduleName)}Result = await this.callZome('create${moduleNameSnakeCase}', ${camelCase(moduleName)});

    return {
      agentPubKey: ${camelCase(moduleName)}Result.agentPubKey,
      ${camelCase(moduleName)}: ${camelCase(moduleName)}Result.${camelCase(moduleName)},
    };
  }

  /**
   * Update my ${camelCase(moduleName)}
   * 
   * @param ${camelCase(moduleName)} the ${camelCase(moduleName)} to create
   * @returns my ${camelCase(moduleName)} with my agentPubKey
   */
  async update${moduleNameTitleCase}(${camelCase(moduleName)}: ${moduleNameTitleCase}): Promise<Agent${moduleNameTitleCase}> {
    const ${camelCase(moduleName)}Result = await this.callZome('update${moduleNameSnakeCase}', ${camelCase(moduleName)});

    return {
      agentPubKey: ${camelCase(moduleName)}Result.agentPubKey,
      ${camelCase(moduleName)}: ${camelCase(moduleName)}Result.${camelCase(moduleName)},
    };
  }

  private callZome(fn_name: string, payload: any) {
    return this.cellClient.callZome(this.zomeName, fn_name, payload);
  }
}
`
});
    