import { PatcherNodeType, PatcherDirectory } from '@patcher/types'; 

import { agentAvatarTs } from './agentAvatarTs';
import { createKebabTs } from './createKebabTs';
import { editKebabTs } from './editKebabTs';
import { holoIdenticonTs } from './holoIdenticonTs';
import { listKebabsTs } from './listKebabsTs';
import { myKebabTs } from './myKebabTs';
import { kebabSingularDetailTs } from './kebabSingularDetailTs';
import { kebabSingularPromptTs } from './kebabSingularPromptTs';
import { searchAgentTs } from './searchAgentTs';
import { updateKebabTs } from './updateKebabTs';
import utils from './utils';  

export default ({moduleNameSnakeCase, moduleNamePluralTitleCase, kebabPlural_, moduleNameTitleCase, moduleNamePlural, moduleName, _kebab, kebabSingular_}: {moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; kebabPlural_: string; moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string; _kebab: string; kebabSingular_: string;}): PatcherDirectory => ({
  type: PatcherNodeType.Directory,
  children: {
  'agent-avatar.ts': agentAvatarTs({moduleNameSnakeCase, moduleNamePluralTitleCase, kebabPlural_, moduleNameTitleCase, moduleNamePlural, moduleName}),
  [`create${_kebab}.ts`]: createKebabTs({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName}),
  [`edit${_kebab}.ts`]: editKebabTs({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName}),
  'holo-identicon.ts': holoIdenticonTs(),
  [`list${_kebab}s.ts`]: listKebabsTs({moduleNamePluralTitleCase, _kebab, kebabPlural_, moduleNamePlural, moduleName}),
  [`my${_kebab}.ts`]: myKebabTs({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural}),
  [`${kebabSingular_}detail.ts`]: kebabSingularDetailTs({moduleNamePluralTitleCase, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName}),
  [`${kebabSingular_}prompt.ts`]: kebabSingularPromptTs({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural}),
  'search-agent.ts': searchAgentTs({moduleNamePluralTitleCase, kebabPlural_, moduleNameTitleCase, moduleNamePlural, moduleName}),
  [`update${_kebab}.ts`]: updateKebabTs({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural, moduleName}),
  'utils': utils()
  }
})