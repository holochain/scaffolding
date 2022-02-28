import { ScFile, ScNodeType } from '@source-craft/types';
import { printTypescript } from '@source-craft/web-apps';
import { FieldDefinition, TypeDefinition } from '@type-craft/vocabulary';
import camelCase from 'lodash-es/camelCase';
import snakeCase from 'lodash-es/snakeCase';
import upperFirst from 'lodash-es/upperFirst';
import flatten from 'lodash-es/flatten';
import { VocabularyElementsImports } from '@type-craft/elements-imports';
import ts from 'typescript';
import { VocabularyTypescriptGenerators } from '@type-craft/typescript';

export function generateCreateTypeVueComponent(
  typescriptGenerators: VocabularyTypescriptGenerators,
  renderersImports: VocabularyElementsImports,
  type: TypeDefinition<any, any>,
  dnaName: string,
  zomeName: string,
): ScFile {
  const createWebComponent = `<template>
  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create ${upperFirst(camelCase(type.name))}</span>

    ${type.fields.map(f => createFieldTemplate(renderersImports, f))}

    <mwc-button 
      label="Create ${upperFirst(camelCase(type.name))}"
      :disabled="!is${upperFirst(camelCase(type.name))}Valid()"
      @click="create${upperFirst(camelCase(type.name))}()"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import '@material/mwc-button';
import { defineComponent } from 'vue';
import { InstalledCell, AppWebsocket, InstalledAppInfo } from '@holochain/client';
import { ${upperFirst(camelCase(type.name))} } from '../../../types/${dnaName}/${zomeName}';
${printTypescript(
  ts.factory.createNodeArray(flatten(type.fields?.map(f => fieldImports(typescriptGenerators, renderersImports, f)))),
)}

export default defineComponent({
  data(): Partial<${upperFirst(camelCase(type.name))}> {
    return {
      ${type.fields?.map(f => `${camelCase(f.name)}: undefined`)}
    }
  },

  methods: {
    is${upperFirst(camelCase(type.name))}Valid() {
      return ${Object.values(type.fields)
        .map(f => `this.${camelCase(f.name)}`)
        .join(' && \n      ')};
    },
    create${upperFirst(camelCase(type.name))}() {
      const cellData = this.appInfo.cell_data.find((c: InstalledCell) => c.role_id === '${dnaName}');

      const ${camelCase(type.name)}: ${upperFirst(camelCase(type.name))} = {
        ${type.fields.map(field => `${camelCase(field.name)}: this.${camelCase(field.name)}`).join(',\n        ')}
      };

      this.appWebsocket.callZome({
        cap_secret: null,
        cell_id: cellData.cell_id,
        zome_name: '${zomeName}',
        fn_name: 'create_${snakeCase(type.name)}',
        payload: ${camelCase(type.name)},
        provenance: cellData.cell_id[1]
      });
    },
  },
  setup() {
    const appWebsocket = inject('appWebsocket') as AppWebsocket;
    const appInfo = inject('appInfo') as InstalledAppInfo;
    return {
      appInfo,
      appWebsocket,
    };
  },
})
</script>`;

  return {
    type: ScNodeType.File,
    content: createWebComponent,
  };
}

function createFieldTemplate(renderersImports: VocabularyElementsImports, field: FieldDefinition<any>): string {
  const fieldRenderers = renderersImports[field.type];
  return `<${fieldRenderers.create.tagName} 
      @change="${camelCase(field.name)} = $event.target.value"
      style="margin-top: 16px"
    ></${fieldRenderers.create.tagName}>`;
}

function fieldImports(
  typescriptGenerators: VocabularyTypescriptGenerators,
  renderersImports: VocabularyElementsImports,
  field: FieldDefinition<any>,
): ts.ImportDeclaration[] {
  return [renderersImports[field.type].create.sideEffectImport, ...typescriptGenerators[field.type].imports].map(
    i => i.importDeclaration,
  );
}
