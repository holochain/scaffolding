import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const kebabPluralServiceMd = ({packageName, moduleNameSnakeCase, moduleNamePluralTitleCase, moduleNameTitleCase}: {packageName: string; moduleNameSnakeCase: string; moduleNamePluralTitleCase: string; moduleNameTitleCase: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `# Frontend Docs >> ${moduleNamePluralTitleCase}Service ||30

The \`${moduleNamePluralTitleCase}Service\` is a state-less class that provides typings wrapping the zome calls that can be made to \`hc_zome${moduleNameSnakeCase}s\`.

\`\`\`js
import { ${moduleNamePluralTitleCase}Service } from '${packageName}';

const service = new ${moduleNamePluralTitleCase}Service(cellClient);

service.getMy${moduleNameTitleCase}().then(my${moduleNameTitleCase} => console.log(my${moduleNameTitleCase}));
\`\`\`

Learn more about the services [here](https://holochain-open-dev.github.io/reusable-modules/frontend/using/#services). `
});
    