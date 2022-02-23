import { ScNodeType, ScDirectory } from '@source-craft/types'; 

import { configTs } from './configTs';
import { contextTs } from './contextTs';
import definitions from './definitions';
import elements from './elements';
import { indexTs } from './indexTs';
import { mocksTs } from './mocksTs';
import { kebabPluralServiceTs } from './kebabPluralServiceTs';
import { kebabPluralStoreTs } from './kebabPluralStoreTs';
import { shimDTs } from './shimDTs';
import { typesTs } from './typesTs';  

export default ({moduleNamePluralTitleCase, moduleNamePlural, moduleNameSnakeCase, kebabPlural_, _kebab, moduleNameTitleCase, kebabSingular_, moduleName}: {moduleNamePluralTitleCase: string; moduleNamePlural: string; moduleNameSnakeCase: string; kebabPlural_: string; _kebab: string; moduleNameTitleCase: string; kebabSingular_: string; moduleName: string;}): ScDirectory => ({
  type: ScNodeType.Directory,
  children: {
  'config.ts': configTs({moduleNamePluralTitleCase, moduleNamePlural}),
  'context.ts': contextTs({moduleNameSnakeCase, moduleNamePluralTitleCase, kebabPlural_, moduleNamePlural}),
  'definitions': definitions({_kebab, moduleNameTitleCase, moduleNamePluralTitleCase, kebabSingular_}),
  'elements': elements({moduleNameSnakeCase, moduleNamePluralTitleCase, kebabPlural_, moduleNameTitleCase, moduleNamePlural, moduleName, _kebab, kebabSingular_}),
  'index.ts': indexTs({_kebab, kebabPlural_, kebabSingular_}),
  'mocks.ts': mocksTs({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase, moduleName}),
  [`${kebabPlural_}service.ts`]: kebabPluralServiceTs({moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase, moduleNamePlural, moduleName}),
  [`${kebabPlural_}store.ts`]: kebabPluralStoreTs({moduleNamePluralTitleCase, kebabPlural_, moduleNameTitleCase, moduleNamePlural, moduleName}),
  'shim.d.ts': shimDTs(),
  'types.ts': typesTs({moduleNamePluralTitleCase, moduleNameTitleCase, moduleName})
  }
})