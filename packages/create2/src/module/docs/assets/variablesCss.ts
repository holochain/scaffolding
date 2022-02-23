import { ScFile, ScNodeType } from '@source-craft/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const variablesCss = (): ScFile => ({
  type: ScNodeType.File,
  content: `html {
  --primary-color: #3f51b5;
  --primary-color-accent: #ff5252;
}

html {
  --mdc-theme-primary: var(--primary-color, #3f51b5);
  --mdc-theme-secondary: var(--primary-color-accent, #ff5252);
  --primary-color-lighter: var(--primary-color, #3f51b5);
  --primary-color-darker: #a22831;
  --primary-color-accent: #ff5252;
  --primary-text-color: #2c3e50;
  --primary-lines-color: #ccc;

  /* Contrast colors */
  --contrast-color-light: #fff;
  --contrast-color-dark: #1d3557;

  /* background-colors */
  --page-background: white;
  --footer-background: rgba(0, 0, 0, 0.1);

  --text-color: black;

  font-family: Arial, Helvetica, sans-serif;
}

html.dark {
  --primary-color: #3f51b5;
  --primary-color-lighter: #e25761;
  --primary-color-darker: #a22831;
  --primary-color-accent: #ff5252;
  --primary-text-color: #eee;

  /* Contrast colors */
  --contrast-color-light: #fff;
  --contrast-color-dark: #1d3557;

  /* background-colors */
  --page-background: #333;
  --footer-background: #4f4f4f;

  --text-color: white;

  --markdown-octicon-link: white;
  --markdown-syntax-background-color: #a0a0a0;
  --markdown-link-color: #fb7881;
  --markdown-blockquote-color: #c9e3ff;
}

@media screen and (min-width: 1024px) {
  .content-area {
    /* max-width: 80vw !important; */
  }

  p {
    max-width: 50rem;
    text-align: justify;
  }

  pre {
    max-width: 50rem;
    text-align: justify;
  }

  ul {
    max-width: 50rem;
    text-align: justify;
  }

  ol {
    max-width: 50rem;
    text-align: justify;
  }

  li {
    max-width: 50rem;
    text-align: justify;
  }

  inline-notification {
    max-width: 50rem;
  }

  .page-slogan {
    max-width: 100%;
  }

  .cli-error {
    background-color: black !important;
    color: white !important;
  }
}

.logo > path {
  fill: #3f51b5;
}
.logo-link > .logo {
  width: 30px;
  height: 30px;
  vertical-align: middle;
}
`
});
    