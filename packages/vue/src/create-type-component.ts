import { ScFile, ScNodeType } from '@source-craft/types';
import { FieldDefinition, TypeDefinition } from '@type-craft/vocabulary';
import camelCase from 'lodash-es/camelCase';
import snakeCase from 'lodash-es/snakeCase';
import upperFirst from 'lodash-es/upperFirst';
import flatten from 'lodash-es/flatten';
import { VocabularyElementsImportDeclarations } from '@type-craft/web-components';
import { VocabularyTypescriptGenerators } from '@type-craft/typescript';
import { kebabCase, uniq } from 'lodash-es';

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

    ${type.fields.map(f => createFieldTemplate(elementsImports, f)).join('\n\n    ')}

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
${uniq(flatten(type.fields?.map(f => fieldImports(typescriptGenerators, elementsImports, f)))).join('\n')}

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
        ${type.fields.map(field => fieldProperty(elementsImports, field)).join('\n        ')}
      };

      const actionHash = await this.appWebsocket.callZome({
        cap_secret: null,
        cell_id: cellData.cell_id,
        zome_name: '${zomeName}',
        fn_name: 'create_${snakeCase(type.name)}',
        payload: ${camelCase(type.name)},
        provenance: cellData.cell_id[1]
      });

      this.$emit('${kebabCase(type.name)}-created', actionHash);
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

function fieldProperty(elementImports: VocabularyElementsImportDeclarations, field: FieldDefinition<any>): string {
  const imports = elementImports[field.type];
  return `${field.name}: this.${camelCase(field.name)}!,${
    imports && imports.create ? '' : `    // TODO: set the ${field.name}`
  }`;
}

function createFieldTemplate(
  elementsImports: VocabularyElementsImportDeclarations,
  field: FieldDefinition<any>,
): string {
  const fieldRenderers = elementsImports[field.type];

  if (!fieldRenderers || !fieldRenderers.create) return '';

  return `<${fieldRenderers.create.tagName} 
      ${Object.entries(field.configuration)
        .map(([configPropName, configValue]) => `${kebabCase(configPropName)}="${configValue}"`)
        .join(' ')}
      @change="${camelCase(field.name)} = $event.target.value"
      style="margin-top: 16px"
    ></${fieldRenderers.create.tagName}>`;
}

function fieldImports(
  typescriptGenerators: VocabularyTypescriptGenerators,
  elementsImports: VocabularyElementsImportDeclarations,
  field: FieldDefinition<any>,
): string[] {
  let imports = [];

  if (typescriptGenerators[field.type]) imports = [...imports, ...typescriptGenerators[field.type].imports];
  if (elementsImports[field.type] && elementsImports[field.type].create)
    imports = [...imports, elementsImports[field.type].create.sideEffectImport];

  return imports.map(i => i.importDeclaration);
}
