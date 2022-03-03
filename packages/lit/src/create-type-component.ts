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
  const createWebComponent = `
import { LitElement, html } from 'lit';
import { state } from 'lit/decorators.js';
import { InstalledCell, AppWebsocket, InstalledAppInfo } from '@holochain/client';
import '@material/mwc-button';
import { ${upperFirst(camelCase(type.name))} } from '../../../types/${dnaName}/${zomeName}';
${uniq(flatten(type.fields?.map(f => fieldImports(typescriptGenerators, elementsImports, f)))).join('\n')}

@customElement('create-${kebabCase(type.name)}')
export class Create${upperFirst(camelCase(type.name))} extends LitElement {

  ${type.fields?.map(
    f => `@state()
  ${camelCase(f.name)}: ${typescriptGenerators[f.type].referenceType} | undefined;`,
  )}

  is${upperFirst(camelCase(type.name))}Valid() {
    return ${Object.values(type.fields)
      .map(f => `this.${camelCase(f.name)}`)
      .join(' && \n      ')};
  },

  @contextProvided({ context: appWebsocketContext })
  appWebsocket!: AppWebsocket;

  @contextProvided({ context: appInfo })
  appInfo!: InstalledAppInfo;

  async create${upperFirst(camelCase(type.name))}() {
    const cellData = this.appInfo.cell_data.find((c: InstalledCell) => c.role_id === '${dnaName}')!;

    const ${camelCase(type.name)}: ${upperFirst(camelCase(type.name))} = {
      ${type.fields.map(field => fieldProperty(elementsImports, field)).join('\n        ')}
    };

    const { entryHash } = await this.appWebsocket.callZome({
      cap_secret: null,
      cell_id: cellData.cell_id,
      zome_name: '${zomeName}',
      fn_name: 'create_${snakeCase(type.name)}',
      payload: ${camelCase(type.name)},
      provenance: cellData.cell_id[1]
    });

    this.dispatchEvent(new CustomEvent('${kebabCase(type.name)}-created', {
      composed: true,
      bubbles: true,
      detail: {
        entryHash
      }
    });
  },

  render() {
    return html\`
      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create ${upperFirst(camelCase(type.name))}</span>

        ${type.fields.map(f => createFieldTemplate(elementsImports, f)).join('\n\n        ')}

        <mwc-button 
          label="Create ${upperFirst(camelCase(type.name))}"
          .disabled=\${!this.is${upperFirst(camelCase(type.name))}Valid()}
          @click=\${() => create${upperFirst(camelCase(type.name))}()}
        ></mwc-button>
    </div>\`;
  }
}
`;

  return {
    type: ScNodeType.File,
    content: createWebComponent,
  };
}

function fieldProperty(elementImports: VocabularyElementsImportDeclarations, field: FieldDefinition<any>): string {
  const imports = elementImports[field.type];
  return `${camelCase(field.name)}: this.${camelCase(field.name)}!,${
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
      field-name="${field.name}"
      @change=\${(e: Event) => this.${camelCase(field.name)} = e.target.value}
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
