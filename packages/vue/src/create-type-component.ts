import { addNpmDependency } from '@source-craft/npm';
import { findByPath, ScDirectory, ScFile, ScNodeType } from '@source-craft/types';
import { printTypescript } from '@source-craft/web-apps';
import { FieldDefinition, getAllChildrenTypes, TypeDefinition, Vocabulary } from '@type-craft/vocabulary';
import camelCase from 'lodash-es/camelCase';
import snakeCase from 'lodash-es/snakeCase';
import upperFirst from 'lodash-es/upperFirst';
import flatten from 'lodash-es/flatten';
import { getAllImports, VocabularyElementsImports } from '@type-craft/elements-imports';
import ts from 'typescript';
import { VocabularyTypescriptGenerators } from '@type-craft/typescript';

const titleCase = (str: string) => upperFirst(camelCase(str));

export function addCreateTypeComponent(
  vueApp: ScDirectory,
  vocabulary: Vocabulary,
  typescriptGenerators: VocabularyTypescriptGenerators,
  renderersImports: VocabularyElementsImports,
  type: TypeDefinition<any, any>,
  dnaName: string,
  zomeName: string,
): ScDirectory {
  const componentFile = generateCreateTypeVueComponent(typescriptGenerators, renderersImports, type, dnaName, zomeName);

  const componentsDir = findByPath(vueApp, 'src/components') as ScDirectory;

  let dnaComponentsDir = findByPath(componentsDir, dnaName) as ScDirectory;

  if (!dnaComponentsDir) {
    dnaComponentsDir = {
      type: ScNodeType.Directory,
      children: {},
    };
    componentsDir.children[dnaName] = dnaComponentsDir;
  }

  let zomeComponentsDir = findByPath(dnaComponentsDir, zomeName) as ScDirectory;

  if (!zomeComponentsDir) {
    zomeComponentsDir = {
      type: ScNodeType.Directory,
      children: {},
    };
    dnaComponentsDir.children[zomeName] = zomeComponentsDir;
  }

  zomeComponentsDir.children[`Create${titleCase(type.name)}.vue`] = componentFile;

  const packageJson = findByPath(vueApp, 'package.json') as ScFile;

  const allTypes = getAllChildrenTypes(vocabulary, type);

  const allRenderers = allTypes.map(t => renderersImports[t]).filter(r => !!r);
  const allImports = flatten(allRenderers.map(r => getAllImports(r)));

  for (const i of allImports) {
    packageJson.content = addNpmDependency(packageJson, i.packageName, i.version).content;
  }

  return vueApp;
}

export function generateCreateTypeVueComponent(
  typescriptGenerators: VocabularyTypescriptGenerators,
  renderersImports: VocabularyElementsImports,
  type: TypeDefinition<any, any>,
  dnaName: string,
  zomeName: string,
): ScFile {
  const createWebComponent = `<template>
  <div style="display: flex; flex-direction: column">
    <span style="font-size: 18px">Create ${titleCase(type.name)}</span>

    ${type.fields.map(f => createFieldTemplate(renderersImports, f))}

    <mwc-button 
      label="Create ${titleCase(type.name)}"
      :disabled="!is${titleCase(type.name)}Valid()"
      @click="create${titleCase(type.name)}"
    ></mwc-button>
  </div>
</template>
<script lang="ts">
import '@material/mwc-button';
import { defineComponent } from 'vue';
import { InstalledCell, AppWebsocket, InstalledAppInfo } from '@holochain/client';
import { ${titleCase(type.name)} } from '../../../types/${dnaName}/${zomeName}';
${printTypescript(
  ts.factory.createNodeArray(flatten(type.fields?.map(f => fieldImports(typescriptGenerators, renderersImports, f)))),
)}

export default defineComponent<Partial<${titleCase(type.name)}>, {
  appWebsocket: AppWebsocket;
  appInfo: InstalledAppInfo;
}>({
  data(): Partial<${titleCase(type.name)}> {
    return {
      ${type.fields?.map(f => `${camelCase(f.name)}: undefined`)}
    }
  },

  methods: {
    is${titleCase(type.name)}Valid() {
      return ${Object.values(type.fields)
        .map(f => `this.${camelCase(f.name)}`)
        .join(' && \n      ')};
    },
    create${titleCase(type.name)}() {
      const cellData = this.appInfo.cell_data.find((c: InstalledCell) => c.role_id === '${dnaName}')!;

      const ${camelCase(type.name)}: ${titleCase(type.name)} = {
        ${type.fields.map(field => `${camelCase(field.name)}: this.${camelCase(field.name)}!`).join(',\n        ')}
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
  inject: ['appInfo', 'appWebsocket']
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
