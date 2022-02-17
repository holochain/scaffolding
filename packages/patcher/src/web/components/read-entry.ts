import { FnDefinition, WebComponent, WebComponentProp } from '@patcher/web-apps';
import { ProgrammingLanguages, TypeDefinition } from '@typecraft/type-definition';
import { ElementContent, Element } from 'hast';
import camelCase from 'lodash-es/camelCase';
import snakeCase from 'lodash-es/snakeCase';

import { titleCase } from '../../utils';
import { appWebsocketContext } from '../appWebsocketContext';

export function readEntryWebComponent(dnaName: string, zomeName: string, type: TypeDefinition<any, any>): WebComponent {
  const template = readTemplate(type);

  const properties: Record<string, WebComponentProp> = {
    entryHash: {
      type: 'string',
    },
  };
  const localState: Record<string, WebComponentProp> = {
    [camelCase(type.name)]: {
      type: `${titleCase(type.name)} | undefined`,
      default: undefined,
    },
    loading: {
      type: 'Boolean',
      default: 'true',
    },
  };
  const onMounted: FnDefinition = {
    async: true,
    imports: [],
    params: [],
    fnContent: `const { appWs, appInfo }: { appWs: AppWebsocket; appInfo: InstalledAppInfo; } = (this as any).holochainContext;
    const cellData = appInfo.cell_data.find((c: InstalledCell) => c.role_id === '${dnaName}')!;
    
    this.${camelCase(type.name)} = await appWs.callZome({
      cap_secret: null,
      cell_id: cellData.cell_id,
      zome_name: '${zomeName}',
      fn_name: 'get_${snakeCase(type.name)}',
      payload: this.entryHash,
      provenance: cellData.cell_id[1]
    });
    
    this.loading = false;`,
  };

  let imports = [
    `import { ${titleCase(type.name)} } from '../../../types/${dnaName}/${zomeName}';`,
    `import '@material/mwc-button';`,
    `import { InstalledCell } from '@holochain/client';`,
  ];

  if (type.fields) {
    for (const field of type.fields) {
      for (const detailEl of field.type.detail) {
        imports.push(`import '${detailEl.customImportDefiningCustomElement}';`);
      }
    }
  }

  return {
    inject: [appWebsocketContext],
    template,
    localState,
    imports,
    properties,
    onMounted,
  };
}

export function readTemplate(type: TypeDefinition<any, any>): ElementContent[] {
  if (!type.fields) return [];

  const fields: Element[] = type.fields.map(f => ({
    type: 'element',
    tagName: f.type.detail[0].tagName,
    inputs: {
      value: `this.${camelCase(type.name)}?.${camelCase(f.name)}`,
    },
    children: [],
  }));
  console.log(JSON.stringify(fields));

  return [
    {
      type: 'ifCondition',
      condition: 'loading',
      then: {
        type: 'element',
        tagName: 'span',
        children: [
          {
            type: 'text',
            value: 'Loading...',
          },
        ],
      },
      else: {
        type: 'element',
        tagName: 'div',
        properties: {
          style: 'display: flex; flex-direction: column',
        },
        children: fields,
      },
    },
  ];
}
