import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const configTs = ({moduleNamePluralTitleCase, moduleNamePlural}: {moduleNamePluralTitleCase: string; moduleNamePlural: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `export interface ${moduleNamePluralTitleCase}Config {
  zomeName: string;
  avatarMode: 'identicon' | 'avatar';
  additionalFields: string[];
  minNicknameLength: number;
}

export const defaultConfig: ${moduleNamePluralTitleCase}Config = {
  zomeName: '${camelCase(moduleNamePlural)}',
  avatarMode: 'avatar',
  additionalFields: [],
  minNicknameLength: 3,
};
`
});
    