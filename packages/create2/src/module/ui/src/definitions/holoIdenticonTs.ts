import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const holoIdenticonTs = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `import { customElement } from 'lit/decorators.js';
import { HoloIdenticon } from '../elements/holo-identicon';

@customElement('holo-identicon')
class HI extends HoloIdenticon {}
`
});
    