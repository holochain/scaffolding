import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const kebabPluralStoreMd = ({packageName, moduleNamePluralTitleCase, moduleNamePlural, moduleName}: {packageName: string; moduleNamePluralTitleCase: string; moduleNamePlural: string; moduleName: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `# Frontend Docs >> ${moduleNamePluralTitleCase}Store ||20

The \`${moduleNamePluralTitleCase}Store\` is a JS class that contains \`svelte\` stores, to which you can subscribe to get reactive updates in your elements.

\`\`\`js
import { ${moduleNamePluralTitleCase}Store } from "${packageName}";

const config = {
  avatarMode: "identicon",
  additionalFields: ["Location", "Bio"], // Custom app level ${moduleName} fields
};
const store = new ${moduleNamePluralTitleCase}Store(cellClient, config);
\`\`\`

> Learn how to setup the \`CellClient\` object [here](https://www.npmjs.com/package/@holochain-open-dev/cell-client).

The config for the \`${moduleNamePluralTitleCase}Store\` has these options:

\`\`\`ts
export interface ${moduleNamePluralTitleCase}Config {
  zomeName: string; // default: '${moduleNamePlural}'
  avatarMode: "identicon" | "avatar"; // default: 'avatar'
  additionalFields: string[]; // default: []
  minNicknameLength: number; // default: 3
}
\`\`\`

Learn more about the stores and how to integrate them in different frameworks [here](https://holochain-open-dev.github.io/reusable-modules/frontend/using/#stores).
`
});
    