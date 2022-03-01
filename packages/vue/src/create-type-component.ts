import { ScFile, ScNodeType } from '@source-craft/types';
import { printTypescript } from '@source-craft/npm';
import { FieldDefinition, TypeDefinition } from '@type-craft/vocabulary';
import camelCase from 'lodash-es/camelCase';
import snakeCase from 'lodash-es/snakeCase';
import upperFirst from 'lodash-es/upperFirst';
import flatten from 'lodash-es/flatten';
import { VocabularyElementsImportDeclarations } from '@type-craft/web-components';
import ts from 'typescript';
import { VocabularyTypescriptGenerators } from '@type-craft/typescript';
import { kebabCase } from 'lodash-es';

export function generateCreateTypeVueComponent(
  typescriptGenerators: VocabularyTypescriptGenerators,
  elementsImports: VocabularyElementsImportDeclarations,
  type: TypeDefinition<any, any>,
  dnaName: string,
  zomeName: string,
): ScFile {
  const createWebComponent = `<template>
  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create ${upperFirst(camelCase(type.name))}</span>

    ${type.fields.map(f => createFieldTemplate(elementsImports, f)).join('\n\n')}

    <mwc-button 
      label="Create ${upperFirst(camelCase(type.name))}"
      :disabled="!is${upperFirst(camelCase(type.name))}Valid()"
      @click="create${upperFirst(camelCase(type.name))}()"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import '@material/mwc-button';
import { defineComponent, inject, ComputedRef } from 'vue';
import { InstalledCell, AppWebsocket, InstalledAppInfo } from '@holochain/client';
import { ${upperFirst(camelCase(type.name))} } from '../../../types/${dnaName}/${zomeName}';
${printTypescript(
  ts.factory.createNodeArray(flatten(type.fields?.map(f => fieldImports(typescriptGenerators, elementsImports, f)))),
)}

export default defineComponent({
  data(): Partial<${upperFirst(camelCase(type.name))}> {
    return {
      ${type.fields?.map(f => `${camelCase(f.name)}: undefined`).join(',\n')}
    }
  },

  methods: {
    is${upperFirst(camelCase(type.name))}Valid() {
      return ${Object.values(type.fields)
        .map(f => `this.${camelCase(f.name)}`)
        .join(' && \n      ')};
    },
    async create${upperFirst(camelCase(type.name))}() {
      const cellData = this.appInfo.cell_data.find((c: InstalledCell) => c.role_id === '${dnaName}')!;

      const ${camelCase(type.name)}: ${upperFirst(camelCase(type.name))} = {
        ${type.fields.map(field => `${camelCase(field.name)}: this.${camelCase(field.name)}!`).join(',\n        ')}
      };

      const { entryHash } = await this.appWebsocket.callZome({
        cap_secret: null,
        cell_id: cellData.cell_id,
        zome_name: '${zomeName}',
        fn_name: 'create_${snakeCase(type.name)}',
        payload: ${camelCase(type.name)},
        provenance: cellData.cell_id[1]
      });

      this.$emit('${kebabCase(type.name)}-created', entryHash)
    },
  },
  emits: ['${kebabCase(type.name)}-created'],
  setup() {
    const appWebsocket = (inject('appWebsocket') as ComputedRef<AppWebsocket>).value;
    const appInfo = (inject('appInfo') as ComputedRef<InstalledAppInfo>).value;
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

function createFieldTemplate(
  elementsImports: VocabularyElementsImportDeclarations,
  field: FieldDefinition<any>,
): string {
  const fieldRenderers = elementsImports[field.type];
  return `<${fieldRenderers.create.tagName} 
      @change="${camelCase(field.name)} = $event.target.value"
      style="margin-top: 16px"
    ></${fieldRenderers.create.tagName}>`;
}

function fieldImports(
  typescriptGenerators: VocabularyTypescriptGenerators,
  elementsImports: VocabularyElementsImportDeclarations,
  field: FieldDefinition<any>,
): ts.ImportDeclaration[] {
  return [elementsImports[field.type].create.sideEffectImport, ...typescriptGenerators[field.type].imports].map(
    i => i.importDeclaration,
  );
}
