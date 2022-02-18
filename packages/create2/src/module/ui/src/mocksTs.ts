import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const mocksTs = ({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase, moduleName}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; moduleNameTitleCase: string; moduleName: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `import { CellClient } from '@holochain-open-dev/cell-client';
import {
  AgentPubKeyB64,
  deserializeHash,
  serializeHash,
} from '@holochain-open-dev/core-types';
import { CellId, AppSignalCb } from '@holochain/client';
import { Agent${moduleNameTitleCase} } from './types';

const sleep = (ms: number) => new Promise(r => setTimeout(() => r(null), ms));

export class ${moduleNamePluralTitleCase}ZomeMock extends CellClient {
  constructor(
    protected agents: Array<Agent${moduleNameTitleCase}> = [],
    protected latency: number = 500
  ) {
    super(null as any, null as any);
  }

  get cellId(): CellId {
    return [
      deserializeHash('uhC0kkSpFl08_2D0Pvw2vEVEkfSgDVZCkyOf1je6qIdClO1o'),
      deserializeHash('uhCAk6oBoqygFqkDreZ0V0bH4R9cTN1OkcEG78OLxVptLWOI'),
    ];
  }

  get myPubKeyB64() {
    return serializeHash(this.cellId[1]);
  }

  create${moduleNameSnakeCase}({ nickname }: { nickname: string }) {
    const agent: Agent${moduleNameTitleCase} = {
      agentPubKey: this.myPubKeyB64,
      ${camelCase(moduleName)}: { nickname, fields: {} },
    };
    this.agents.push(agent);

    return agent;
  }

  search${moduleNameSnakeCase}s({ nicknamePrefix }: { nicknamePrefix: string }) {
    return this.agents.filter(a =>
      a.${camelCase(moduleName)}.nickname.startsWith(nicknamePrefix.slice(0, 3))
    );
  }

  get_my${moduleNameSnakeCase}() {
    const agent = this.findAgent(this.myPubKeyB64);

    if (!agent) return undefined;
    return {
      agentPubKey: agent.agentPubKey,
      ${camelCase(moduleName)}: agent ? agent.${camelCase(moduleName)} : undefined,
    };
  }

  get_agent${moduleNameSnakeCase}(agent_address: AgentPubKeyB64) {
    const agent = this.findAgent(agent_address);
    return agent ? agent : undefined;
  }

  get_all${moduleNameSnakeCase}s() {
    return this.agents;
  }

  findAgent(agent_address: AgentPubKeyB64) {
    return this.agents.find(user => user.agentPubKey === agent_address);
  }

  async callZome(
    zomeName: string,
    fnName: string,
    payload: any,
    timeout?: number
  ): Promise<any> {
    await sleep(this.latency);
    return (this as any)[fnName](payload);
  }
  addSignalHandler(signalHandler: AppSignalCb): { unsubscribe: () => void } {
    throw new Error('Method not implemented.');
  }
}
`
});
    