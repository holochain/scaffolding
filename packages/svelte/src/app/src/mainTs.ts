import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const mainTs = (): ScFile => ({
  type: ScNodeType.File,
  content: `import '@webcomponents/scoped-custom-element-registry';
import App from './App.svelte';

const app = new App({
  target: document.body,
});

export default app;
`
});
    