import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { agentAvatarTs } from './agentAvatarTs';
import { createKebabTs } from './createKebabTs';
import { holoIdenticonTs } from './holoIdenticonTs';
import { listKebabsTs } from './listKebabsTs';
import { myKebabTs } from './myKebabTs';
import { kebabSingularDetailTs } from './kebabSingularDetailTs';
import { kebabSingularPromptTs } from './kebabSingularPromptTs';
import { searchAgentTs } from './searchAgentTs';
import { updateKebabTs } from './updateKebabTs';  

export default ({_kebab, moduleNameTitleCase, moduleNamePluralTitleCase, kebabSingular_}: {_kebab: string; moduleNameTitleCase: string; moduleNamePluralTitleCase: string; kebabSingular_: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'agent-avatar.ts': agentAvatarTs(),
  [`create${_kebab}.ts`]: createKebabTs({_kebab, moduleNameTitleCase}),
  'holo-identicon.ts': holoIdenticonTs(),
  [`list${_kebab}s.ts`]: listKebabsTs({moduleNamePluralTitleCase, _kebab}),
  [`my${_kebab}.ts`]: myKebabTs({_kebab, moduleNameTitleCase}),
  [`${kebabSingular_}detail.ts`]: kebabSingularDetailTs({kebabSingular_, moduleNameTitleCase}),
  [`${kebabSingular_}prompt.ts`]: kebabSingularPromptTs({kebabSingular_, moduleNameTitleCase}),
  'search-agent.ts': searchAgentTs(),
  [`update${_kebab}.ts`]: updateKebabTs({_kebab, moduleNameTitleCase})
  }
})