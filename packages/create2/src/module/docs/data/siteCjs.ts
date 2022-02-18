import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const siteCjs = ({packageName, moduleNamePlural}: {packageName: string; moduleNamePlural: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `module.exports = async function () {
  return {
    name: "${packageName}",
    description:
      "A Holochain module to handle ${moduleNamePlural} with at least a nickname",
    socialLinks: [
      {
        name: "GitHub",
        url: "https://github.com/holochain-open-dev/${moduleNamePlural}/",
      },
    ],
    gitSiteUrl: "https://github.com/holochain-open-dev/${moduleNamePlural}",
    gitBranch: "main",
    helpUrl: "https://github.com/holochain-open-dev/${moduleNamePlural}",
    logoAlt: "A gym tool",
    socialMediaImage: "/_assets/holochain-gym.png",
  };
};
`
});
    