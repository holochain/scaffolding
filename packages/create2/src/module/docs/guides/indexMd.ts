import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const indexMd = ({moduleNameTitleCase, moduleNamePlural, moduleName}: {moduleNameTitleCase: string; moduleNamePlural: string; moduleName: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `# Guides

The ${moduleNamePlural} zome and its accompanying frontend module are designed to implement and export useful functionality around personal ${moduleName} information about the agents in a Holochain DHT.

The only field that this module assumes 

Existing functionalities:

- Creating a ${moduleName}.
- Updating a ${moduleName}.
- Searching agents by nickname.
- Getting the ${moduleName} for a list of agents.

Future functionality will include:

- Configurable ${moduleName} fields.
- ${moduleNameTitleCase} detail frontend element.

> In the future, when the personas & ${moduleNamePlural} application is fully developed, this module will switch to storing data in it, and will serve only as a bridge to get that private data. We hope to maintain the modules and their interfaces as similar as they are now, and that the migration friction is low.

Get started with adding the module into your Holochain app by reading the [Setting Up section](./setting-up/adding-the-zome.md).`
});
    