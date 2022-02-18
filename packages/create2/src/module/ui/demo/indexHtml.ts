import { PatcherFile, PatcherNodeType } from '@patcher/types';
import camelCase from 'lodash-es/camelCase';
import kebabCase from 'lodash-es/kebabCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';

export const indexHtml = ({moduleNamePluralTitleCase, _kebab, kebabPlural_, kebabSingular_, moduleNameTitleCase, moduleNamePlural}: {moduleNamePluralTitleCase: string; _kebab: string; kebabPlural_: string; kebabSingular_: string; moduleNameTitleCase: string; moduleNamePlural: string;}): PatcherFile => ({
  type: PatcherNodeType.File,
  content: `<!DOCTYPE html>
<html lang="en-GB">
  <head>
    <meta charset="utf-8" />
    <style>
      body {
        background: #fafafa;
        font-family: Arial, Helvetica, sans-serif;
      }
    </style>
    <link
      href="https://fonts.googleapis.com/css?family=Roboto:300,400,500"
      rel="stylesheet"
    />
    <link
      href="https://fonts.googleapis.com/css?family=Material+Icons&display=block"
      rel="stylesheet"
    />
  </head>
  <body>
    <${kebabPlural_}test></${kebabPlural_}test>

    <script type="module">
      import {
        ${moduleNameTitleCase}Prompt,
        SearchAgent,
        ${moduleNamePluralTitleCase}Store,
        ${moduleNamePlural}StoreContext,
        ${moduleNameTitleCase}Detail,
        My${moduleNameTitleCase},
      } from '../dist';
      import { HolochainClient } from '@holochain-open-dev/cell-client';
      import { LitElement, html } from 'lit';
      import { ScopedElementsMixin } from '@open-wc/scoped-elements';
      import {
        provide,
        ContextProviderElement,
      } from '@holochain-open-dev/context';

      class ${moduleNamePluralTitleCase}Test extends ScopedElementsMixin(LitElement) {
        static get properties() {
          return {
            loaded: {
              type: Boolean,
            },
          };
        }

        async firstUpdated() {
          const client = await HolochainClient.connect(
            \`ws://localhost:\${process.env.HC_PORT}\`,
            'test-app'
          );
          const cellClient = client.forCell(
            client.cellDataByRoleId('${moduleNamePlural}')
          );

          this.store = new ${moduleNamePluralTitleCase}Store(cellClient, {
            avatarMode: 'avatar',
            additionalFields: ['bio', 'location'],
          });
          this.loaded = true;
        }

        render() {
          if (!this.loaded) return html\`<span>Loading...</span>\`;
          return html\`
            <context-provider
              .context=\${${moduleNamePlural}StoreContext}
              .value=\${this.store}
            >
              <${kebabSingular_}prompt>
                <my${_kebab} style="height: 800px; width: 800px;"></my${_kebab}>
                <search-agent include-myself></search-agent>
              </${kebabSingular_}prompt>
            </context-provider>
          \`;
        }

        static get scopedElements() {
          return {
            '${kebabSingular_}prompt': ${moduleNameTitleCase}Prompt,
            'my${_kebab}': My${moduleNameTitleCase},
            'search-agent': SearchAgent,
            'context-provider': ContextProviderElement,
          };
        }
      }

      customElements.define('${kebabPlural_}test', ${moduleNamePluralTitleCase}Test);
    </script>
  </body>
</html>
`
});
    