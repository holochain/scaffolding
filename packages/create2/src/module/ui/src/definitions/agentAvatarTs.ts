import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const agentAvatarTs = (): ScFile => ({
  type: ScNodeType.File,
  content: `import { customElement } from 'lit/decorators.js';
import { AgentAvatar } from '../elements/agent-avatar';

@customElement('agent-avatar')
class AA extends AgentAvatar {}
`
});
    