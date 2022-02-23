import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const $49$0DefaultsNjk = (): ScFile => ({
  type: ScNodeType.File,
  content: `{% set _pageTitle = title %}
{% if title != site.name %}
  {% set _pageTitle = title + ': ' + site.name %}
{% endif %}
{% if pageTitle %}
  {% set _pageTitle = pageTitle %}
{% endif %}

<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{{ _pageTitle }}</title>
<meta property="og:title" content="{{ _pageTitle }}"/>

<meta name="generator" content="rocket 0.1">
<link rel="canonical" href="{{ page.url }}"/>

{% set _description = site.description %}
{% if description %}
  {% set _description = description %}
{% endif %}
<meta name="description" content="{{ _description }}"/>
<meta property="og:description" content="{{ _description }}"/>

<link
  rel="stylesheet"
  href="https://cdn.jsdelivr.net/npm/@shoelace-style/shoelace@2.0.0-beta.64/dist/themes/light.css"
/>
<script
  type="module"
  src="https://cdn.jsdelivr.net/npm/@shoelace-style/shoelace@2.0.0-beta.64/dist/shoelace.js"
></script>
`
});
    