import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const indexJs = ({moduleNamePluralTitleCase, moduleNamePlural}: {moduleNamePluralTitleCase: string; moduleNamePlural: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `import ConductorApi from '@holochain/conductor-api';

import { AppWebsocketMock, DnaMock } from 'holochain-ui-test-utils';
import { ${moduleNamePluralTitleCase}Mock } from './${moduleNamePlural}.mock';

/**
 * If process.env.CONDUCTOR_URL is undefined, it will mock the backend
 * If process.env.CONDUCTOR_URL is defined, it will try to connect to holochain at that url
 */
const dnaMock = new DnaMock({
  ${moduleNamePlural}: new ${moduleNamePluralTitleCase}Mock(),
});
export async function getAppWebsocket() {
  if (process.env.CONDUCTOR_URL)
    return ConductorApi.AppWebsocket.connect(process.env.CONDUCTOR_URL);
  else {
    return new AppWebsocketMock([dnaMock]);
  }
}
`
});
    