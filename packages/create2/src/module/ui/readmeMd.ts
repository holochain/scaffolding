import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const readmeMd = ({packageName, moduleNameSnakeCase, moduleNamePlural}: {packageName: string; moduleNameSnakeCase: string; moduleNamePlural: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `# ${packageName}

Frontend module for the Holochain zome \`hc_zome${moduleNameSnakeCase}s\`.

This package includes types, a service and a store, and a collection of Custom Elements to build Holochain applications that automatically connect and interact with the \`hc_zome${moduleNameSnakeCase}s\` zome. 

By using [Custom Elements](https://developers.google.com/web/fundamentals/web-components/customelements), this package exports frontend blocks reusable in any framework, that make it really easy for consuming web applications to include functionality to create and update ${moduleNamePlural}, or search for an agent in the DHT.

Read about how to include both the zome and this frontend module in your application here:

- https://holochain-open-dev.github.io/${moduleNamePlural}`
});
    