import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const indexMd = ({packageName, moduleNamePlural}: {packageName: string; moduleNamePlural: string;}): ScFile => ({
  type: ScNodeType.File,
  content: `---
title: "${packageName}"
layout: layout-home
slogan: A Holochain module to handle ${moduleNamePlural} with at least a nickname
callToActionItems: [{ text: "Guides and Docs", href: "/guides/" }]
---
`
});
    