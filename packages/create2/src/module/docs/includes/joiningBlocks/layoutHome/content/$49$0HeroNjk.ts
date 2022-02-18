import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const $49$0HeroNjk = (): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `<svg class="page-logo logo" viewBox="0 0 30 30" xmlns="http://www.w3.org/2000/svg">
 <defs>
  <linearGradient id="a" x1="21.375" x2="32.5" y1="23.879" y2="25.504" gradientUnits="userSpaceOnUse">
   <stop stop-color="#f57900" offset="0"/>
   <stop stop-color="#c66100" offset="1"/>
  </linearGradient>
 </defs>
 <g transform="matrix(1.4249 0 0 1.4249 -6.2099 -8.9348)">
  <path d="m15.31954,8.36558c-.0969.01294-.19006.04582-.27361.09657l-7.21053,4.12031c-.2187.12609-.35364.35916-.35409.61161v7.21053c-.0008.24029.12043.46456.3219.59551l6.18046,4.12031c.22405.14442.51025.15064.74037.01609l7.21054-4.12031c.2187-.12609.35364-.35917.35409-.61161v-7.21053c.0008-.24029-.12043-.46456-.3219-.59551l-6.18046-4.12031c-.13769-.09082-.30279-.13067-.46675-.11266z" fill="#000000" />
  <g transform="matrix(.45216 0 0 .45216 1.8173 2.1004)">
   <path d="m15.901 25.504 12 8v14l-12-8v-14z" style="fill: var(--primary-color); opacity: 0.6"/>
   <path d="m29.901 17.504 12 8-14 8-12-8 14-8z" style="fill: var(--primary-color); opacity: 0.8"/>
   <path d="m41.901 25.504v14l-14 8v-14l14-8z" style="fill: var(--primary-color); opacity: 1.0"/>
  </g>
 </g>
</svg>

{% if rocketLaunch.homeLayout === 'background' %}
  <img class="page-background" src="{{ '/_assets/home-background.svg' | asset | url }}" role="presentation"/>
{% endif %}

<h1 class="page-title">{{title}}</h1>

<p class="page-slogan">{{slogan}}</p>

<div class="call-to-action-list" role="list">
  {% for callToAction in callToActionItems %}
    <a class="call-to-action" href="{{ callToAction.href | url }}" role="listitem">{{ callToAction.text | safe }}</a>
  {% endfor %}
</div>
`
});
    