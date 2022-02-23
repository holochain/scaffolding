import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const webmanifestJson = ({moduleNamePluralTitleCase, kebabPlural_}: {moduleNamePluralTitleCase: string; kebabPlural_: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `{
  "name": "${moduleNamePluralTitleCase} Module",
  "short_name": "${kebabPlural_}module",
  "theme_color": "#3f51b5",
  "background_color": "#1d3557",
  "display": "standalone",
  "orientation": "portrait",
  "Scope": "/",
  "start_url": "/",
  "icons": [],
  "splash_pages": null
}
`
});
    