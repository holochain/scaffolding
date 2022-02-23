import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const footerJson = (): ScFile => ({
  type: ScNodeType.File,
  content: `[
  {
    "name": "Holochain",
    "children": [
      {
        "text": "Documentation",
        "href": "https://developer.holochain.org/docs/references/"
      },
      {
        "text": "Holochain Gym",
        "href": "https://holochain-gym.github.io"
      },
      {
        "text": "Forum",
        "href": "https://forum.holochain.org"
      }
    ]
  },
  {
    "name": "Discover",
    "children": [
      {
        "text": "Holochain Open Dev Blog",
        "href": "https://holochain-open-dev.github.io/blog"
      },
      {
        "text": "Open Source Modules",
        "href": "https://github.com/holochain-open-dev"
      }
    ]
  },
  {
    "name": "License",
    "children": [
      {
        "text": "This work is licensed under",
        "href": "http://creativecommons.org/licenses/by/4.0/"
      },
      {
        "text": "a Creative \\nCommons Attribution 4.0",
        "href": "http://creativecommons.org/licenses/by/4.0/"
      },
      {
        "text": "International License",
        "href": "http://creativecommons.org/licenses/by/4.0/"
      }
    ]
  }
]
`
});
    