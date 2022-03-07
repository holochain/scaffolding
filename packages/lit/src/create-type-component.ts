import { ScFile, ScNodeType } from '@source-craft/types';
import { FieldDefinition, TypeDefinition } from '@type-craft/vocabulary';
import camelCase from 'lodash-es/camelCase';
import snakeCase from 'lodash-es/snakeCase';
import upperFirst from 'lodash-es/upperFirst';
import flatten from 'lodash-es/flatten';
import { VocabularyElementsImportDeclarations } from '@type-craft/web-components';
import { VocabularyTypescriptGenerators } from '@type-craft/typescript';
import { kebabCase, uniq } from 'lodash-es';

export function generateCreateTypeLitComponent(
  typescriptGenerators: VocabularyTypescriptGenerators,
  elementsImports: VocabularyElementsImportDeclarations,
  type: TypeDefinition<any, any>,
  dnaName: string,
  zomeName: string,
): ScFile {
  const createWebComponent = `
import { LitElement, html } from 'lit';
import { state, customElement } from 'lit/decorators.js';
import { InstalledCell, AppWebsocket, InstalledAppInfo } from '@holochain/client';
import { contextProvided } from '@holochain-open-dev/context';
import { appWebsocketContext, appInfoContext } from '../../../contexts';
import { ${upperFirst(camelCase(type.name))} } from '../../../types/${dnaName}/${zomeName}';
import '@material/mwc-button';
${uniq(flatten(type.fields?.map(f => fieldImports(typescriptGenerators, elementsImports, f)))).join('\n')}

@customElement('create-${kebabCase(type.name)}')
export class Create${upperFirst(camelCase(type.name))} extends LitElement {

  ${type.fields?.map(
    f => `@state()
  _${camelCase(f.name)}: ${typescriptGenerators[f.type].referenceType} | undefined;`,
  )}

  is${upperFirst(camelCase(type.name))}Valid() {
    return ${Object.values(type.fields)
      .map(f => `this._${camelCase(f.name)}`)
      .join(' && \n      ')};
  }

  @contextProvided({ context: appWebsocketContext })
  appWebsocket!: AppWebsocket;

  @contextProvided({ context: appInfoContext })
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
    }));
  }

  render() {
    return html\`
      <div style="display: flex; flex-direction: column">
        <span style="font-size: 18px">Create ${upperFirst(camelCase(type.name))}</span>

        ${type.fields.map(f => createFieldTemplate(elementsImports, f)).join('\n\n        ')}

        <mwc-button 
          label="Create ${upperFirst(camelCase(type.name))}"
          .disabled=\${!this.is${upperFirst(camelCase(type.name))}Valid()}
          @click=\${() => this.create${upperFirst(camelCase(type.name))}()}
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
  return `${camelCase(field.name)}: this._${camelCase(field.name)}!,${
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
        .map(([configPropName, configValue]) => `${configPropName}="${configValue}"`)
        .join(' ')}
      @change=\${(e: Event) => this._${camelCase(field.name)} = (e.target as any).value}
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
