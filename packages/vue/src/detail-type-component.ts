import { ScFile, ScNodeType } from '@source-craft/types';
import { printTypescript } from '@source-craft/npm';
import { VocabularyElementsImportDeclarations } from '@type-craft/web-components';
import { VocabularyTypescriptGenerators } from '@type-craft/typescript';
import { FieldDefinition, TypeDefinition } from '@type-craft/vocabulary';
import { camelCase, flatten, snakeCase, upperFirst } from 'lodash-es';
import ts from 'typescript';

export function generateTypeDetailVueComponent(
  typescriptGenerators: VocabularyTypescriptGenerators,
  elementsImports: VocabularyElementsImportDeclarations,
  type: TypeDefinition<any, any>,
  dnaName: string,
  zomeName: string,
): ScFile {
  const detailWebComponent = `<template>
  <div v-if="${camelCase(type.name)}" style="display: flex; flex-direction: column">
    <span style="font-size: 18px">${upperFirst(camelCase(type.name))}</span>

${type.fields.map(f => fieldDetailTemplate(type.name, elementsImports, f))}

  </div>
  <div v-else style="display: flex; flex: 1; align-items: center; justify-content: center">
    <mwc-circular-progress indeterminate></mwc-circular-progress>
  </div>
</template>
<script lang="ts">
import { defineComponent, inject, ComputedRef } from 'vue';
import { InstalledCell, AppWebsocket, InstalledAppInfo } from '@holochain/client';
import { ${upperFirst(camelCase(type.name))} } from '../../../types/${dnaName}/${zomeName}';
${printTypescript(
  ts.factory.createNodeArray(flatten(type.fields?.map(f => fieldImports(typescriptGenerators, elementsImports, f)))),
)}
import '@material/mwc-circular-progress';

export default defineComponent({
  props: {
    entryHash: {
      type: String,
      required: true
    }
  },
  data(): { ${camelCase(type.name)}: ${upperFirst(camelCase(type.name))} | undefined } {
    return {
      ${camelCase(type.name)}: undefined
    }
  },
  async mounted() {
    const cellData = this.appInfo.cell_data.find((c: InstalledCell) => c.role_id === '${dnaName}')!;

    this.${camelCase(type.name)} = await this.appWebsocket.callZome({
      cap_secret: null,
      cell_id: cellData.cell_id,
      zome_name: '${zomeName}',
      fn_name: 'get_${snakeCase(type.name)}',
      payload: this.entryHash,
      provenance: cellData.cell_id[1]
    });
  },
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
    content: detailWebComponent,
  };
}

function fieldDetailTemplate(
  typeName: string,
  elementsImports: VocabularyElementsImportDeclarations,
  field: FieldDefinition<any>,
): string {
  const fieldRenderers = elementsImports[field.type];
  return `
    <${fieldRenderers.detail.tagName}
      :value="${camelCase(typeName)}.${camelCase(field.name)}"
      style="margin-top: 16px"
    ></${fieldRenderers.detail.tagName}>`;
}

function fieldImports(
  typescriptGenerators: VocabularyTypescriptGenerators,
  elementsImports: VocabularyElementsImportDeclarations,
  field: FieldDefinition<any>,
): ts.ImportDeclaration[] {
  return [elementsImports[field.type].detail.sideEffectImport, ...typescriptGenerators[field.type].imports].map(
    i => i.importDeclaration,
  );
}
