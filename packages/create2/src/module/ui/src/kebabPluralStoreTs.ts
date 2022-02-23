import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const kebabPluralStoreTs = ({moduleNamePluralTitleCase, kebabPlural_, moduleNameTitleCase, moduleNamePlural, moduleName}: {moduleNamePluralTitleCase: string; kebabPlural_: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `import { CellClient } from '@holochain-open-dev/cell-client';
import {
  AgentPubKeyB64,
  Dictionary,
  serializeHash,
} from '@holochain-open-dev/core-types';
import merge from 'lodash-es/merge';

import { ${moduleNamePluralTitleCase}Service } from './${kebabPlural_}service';
import { Agent${moduleNameTitleCase}, ${moduleNameTitleCase} } from './types';
import { writable, Writable, derived, Readable, get } from 'svelte/store';
import { defaultConfig, ${moduleNamePluralTitleCase}Config } from './config';

export class ${moduleNamePluralTitleCase}Store {
  /** Private */
  private _service: ${moduleNamePluralTitleCase}Service;
  private _known${moduleNamePluralTitleCase}Store: Writable<Dictionary<${moduleNameTitleCase}>> = writable({});

  /** Static info */
  public myAgentPubKey: AgentPubKeyB64;

  /** Readable stores */

  // Store containing all the ${camelCase(moduleNamePlural)} that have been fetched
  // The key is the agentPubKey of the agent
  public known${moduleNamePluralTitleCase}: Readable<Dictionary<${moduleNameTitleCase}>> = derived(
    this._known${moduleNamePluralTitleCase}Store,
    i => i
  );

  // Store containing my ${camelCase(moduleName)}
  public my${moduleNameTitleCase}: Readable<${moduleNameTitleCase}> = derived(
    this._known${moduleNamePluralTitleCase}Store,
    ${camelCase(moduleNamePlural)} => ${camelCase(moduleNamePlural)}[this.myAgentPubKey]
  );

  // Returns a store with the ${camelCase(moduleName)} of the given agent
  ${camelCase(moduleName)}Of(agentPubKey: AgentPubKeyB64): Readable<${moduleNameTitleCase}> {
    return derived(this._known${moduleNamePluralTitleCase}Store, ${camelCase(moduleNamePlural)} => ${camelCase(moduleNamePlural)}[agentPubKey]);
  }

  config: ${moduleNamePluralTitleCase}Config;

  constructor(
    protected cellClient: CellClient,
    config: Partial<${moduleNamePluralTitleCase}Config> = {}
  ) {
    this.config = merge(defaultConfig, config);
    this._service = new ${moduleNamePluralTitleCase}Service(cellClient, this.config.zomeName);
    this.myAgentPubKey = serializeHash(cellClient.cellId[1]);
  }

  /** Actions */

  /**
   * Fetches the ${camelCase(moduleNamePlural)} for all agents in the DHT
   *
   * You can subscribe to \`know${moduleNamePluralTitleCase}\` to get updated with all the ${camelCase(moduleNamePlural)} when this call is done
   *
   * Warning! Can be very slow
   */
  async fetchAll${moduleNamePluralTitleCase}(): Promise<void> {
    const all${moduleNamePluralTitleCase} = await this._service.getAll${moduleNamePluralTitleCase}();

    this._known${moduleNamePluralTitleCase}Store.update(${camelCase(moduleNamePlural)} => {
      for (const ${camelCase(moduleName)} of all${moduleNamePluralTitleCase}) {
        ${camelCase(moduleNamePlural)}[${camelCase(moduleName)}.agentPubKey] = ${camelCase(moduleName)}.${camelCase(moduleName)};
      }
      return ${camelCase(moduleNamePlural)};
    });
  }

  /**
   * Fetches the ${camelCase(moduleName)} for the given agent
   */
  async fetchAgent${moduleNameTitleCase}(
    agentPubKey: AgentPubKeyB64
  ): Promise<${moduleNameTitleCase} | undefined> {
    // For now, optimistic return of the cached ${camelCase(moduleName)}
    // TODO: implement cache invalidation

    const known${moduleNamePluralTitleCase} = get(this._known${moduleNamePluralTitleCase}Store);

    if (known${moduleNamePluralTitleCase}[agentPubKey]) return known${moduleNamePluralTitleCase}[agentPubKey];

    const ${camelCase(moduleName)} = await this._service.getAgent${moduleNameTitleCase}(agentPubKey);

    if (!${camelCase(moduleName)}) return;

    this._known${moduleNamePluralTitleCase}Store.update(${camelCase(moduleNamePlural)} => {
      ${camelCase(moduleNamePlural)}[${camelCase(moduleName)}.agentPubKey] = ${camelCase(moduleName)}.${camelCase(moduleName)};
      return ${camelCase(moduleNamePlural)};
    });
    return ${camelCase(moduleName)}.${camelCase(moduleName)};
  }

  /**
   * Fetches the ${camelCase(moduleNamePlural)} for the given agents in the DHT
   *
   * You can subscribe to know${moduleNamePluralTitleCase} to get updated with all the ${camelCase(moduleNamePlural)} when this call is done
   *
   * Use this over \`fetchAgent${moduleNameTitleCase}\` when fetching multiple ${camelCase(moduleNamePlural)}, as it will be more performant
   */
  async fetchAgents${moduleNamePluralTitleCase}(agentPubKeys: AgentPubKeyB64[]): Promise<void> {
    // For now, optimistic return of the cached ${camelCase(moduleName)}
    // TODO: implement cache invalidation

    const known${moduleNamePluralTitleCase} = get(this._known${moduleNamePluralTitleCase}Store);

    const agentsWeAlreadKnow = Object.keys(known${moduleNamePluralTitleCase});
    const ${camelCase(moduleNamePlural)}ToFetch = agentPubKeys.filter(
      pubKey => !agentsWeAlreadKnow.includes(pubKey)
    );

    if (${camelCase(moduleNamePlural)}ToFetch.length === 0) {
      return;
    }

    const fetched${moduleNamePluralTitleCase} = await this._service.getAgents${moduleNamePluralTitleCase}(
      ${camelCase(moduleNamePlural)}ToFetch
    );

    this._known${moduleNamePluralTitleCase}Store.update(${camelCase(moduleNamePlural)} => {
      for (const fetched${moduleNameTitleCase} of fetched${moduleNamePluralTitleCase}) {
        ${camelCase(moduleNamePlural)}[fetched${moduleNameTitleCase}.agentPubKey] = fetched${moduleNameTitleCase}.${camelCase(moduleName)};
      }
      return ${camelCase(moduleNamePlural)};
    });
  }

  /**
   * Fetch my ${camelCase(moduleName)}
   *
   * You can subscribe to \`my${moduleNameTitleCase}\` to get updated with my ${camelCase(moduleName)}
   */
  async fetchMy${moduleNameTitleCase}(): Promise<void> {
    const ${camelCase(moduleName)} = await this._service.getMy${moduleNameTitleCase}();
    if (${camelCase(moduleName)}) {
      this._known${moduleNamePluralTitleCase}Store.update(${camelCase(moduleNamePlural)} => {
        ${camelCase(moduleNamePlural)}[${camelCase(moduleName)}.agentPubKey] = ${camelCase(moduleName)}.${camelCase(moduleName)};
        return ${camelCase(moduleNamePlural)};
      });
    }
  }

  /**
   * Search the ${camelCase(moduleNamePlural)} for the agent with nicknames starting with the given nicknamePrefix
   *
   * @param nicknamePrefix must be of at least 3 characters
   * @returns the ${camelCase(moduleNamePlural)} with the nickname starting with nicknamePrefix
   */
  async search${moduleNamePluralTitleCase}(nicknamePrefix: string): Promise<Agent${moduleNameTitleCase}[]> {
    const searched${moduleNamePluralTitleCase} = await this._service.search${moduleNamePluralTitleCase}(nicknamePrefix);

    this._known${moduleNamePluralTitleCase}Store.update(${camelCase(moduleNamePlural)} => {
      for (const ${camelCase(moduleName)} of searched${moduleNamePluralTitleCase}) {
        ${camelCase(moduleNamePlural)}[${camelCase(moduleName)}.agentPubKey] = ${camelCase(moduleName)}.${camelCase(moduleName)};
      }
      return ${camelCase(moduleNamePlural)};
    });
    return searched${moduleNamePluralTitleCase};
  }

  /**
   * Create my ${camelCase(moduleName)}
   *
   * Note that there is no guarantee on nickname uniqness
   *
   * @param ${camelCase(moduleName)} ${camelCase(moduleName)} to be created
   */
  async create${moduleNameTitleCase}(${camelCase(moduleName)}: ${moduleNameTitleCase}): Promise<void> {
    await this._service.create${moduleNameTitleCase}(${camelCase(moduleName)});

    this._known${moduleNamePluralTitleCase}Store.update(${camelCase(moduleNamePlural)} => {
      ${camelCase(moduleNamePlural)}[this.myAgentPubKey] = ${camelCase(moduleName)};
      return ${camelCase(moduleNamePlural)};
    });
  }

  /**
   * Update my ${camelCase(moduleName)}
   *
   * @param ${camelCase(moduleName)} ${camelCase(moduleName)} to be created
   */
  async update${moduleNameTitleCase}(${camelCase(moduleName)}: ${moduleNameTitleCase}): Promise<void> {
    await this._service.update${moduleNameTitleCase}(${camelCase(moduleName)});

    this._known${moduleNamePluralTitleCase}Store.update(${camelCase(moduleNamePlural)} => {
      ${camelCase(moduleNamePlural)}[this.myAgentPubKey] = ${camelCase(moduleName)};
      return ${camelCase(moduleNamePlural)};
    });
  }
}
`
});
    