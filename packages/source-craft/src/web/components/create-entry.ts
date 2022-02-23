import { FnDefinition, WebComponent, WebComponentProp } from '@source-craft/web-apps';
import { ProgrammingLanguages, TypeDefinition } from '@type-craft/vocabulary';
import { ElementContent, Element } from 'hast';
import camelCase from 'lodash-es/camelCase';
import snakeCase from 'lodash-es/snakeCase';

import { titleCase } from '../../utils';
import { appWebsocketContext } from '../appWebsocketContext';

export function createEntryWebComponent(
  dnaName: string,
  zomeName: string,
  type: TypeDefinition<any, any>,
): WebComponent {
  const template = createTemplate(type);

  const methods: Record<string, FnDefinition> = {};
  const localState: Record<string, WebComponentProp> = {};
  let imports = [
    `import { ${titleCase(type.name)} } from '../../../types/${dnaName}/${zomeName}';`,
    `import '@material/mwc-button';`,
  ];

  if (type.fields) {
    for (const field of type.fields) {
      const fieldState = camelCase(field.name);
      methods[`on${titleCase(field.name)}Change`] = {
        async: false,
        imports: [],
        params: [{ name: 'event', type: 'Event' }],
        fnContent: `this.${fieldState} = (event.target as any).value;`,
      };
      localState[fieldState] = {
        type: `${field.type.generators[ProgrammingLanguages.Typescript].referenceType} | undefined`,
        default: 'undefined',
      };

      for (const createEl of field.type.create) {
        imports.push(`import '${createEl.customImportDefiningCustomElement}';`);
      }
    }

    methods[`create${titleCase(type.name)}`] = {
      async: true,
      imports: [`import { InstalledCell } from '@holochain/client';`],
      params: [],
      fnContent: `const { appWs, appInfo }: { appWs: AppWebsocket; appInfo: InstalledAppInfo; } = (this as any).holochainContext;
      const cellData = appInfo.cell_data.find((c: InstalledCell) => c.role_id === '${dnaName}')!;

      const ${camelCase(type.name)}: ${titleCase(type.name)} = {
        ${type.fields.map(field => `${camelCase(field.name)}: this.${camelCase(field.name)}!`).join(',\n        ')}
      };

      await appWs.callZome({
        cap_secret: null,
        cell_id: cellData.cell_id,
        zome_name: '${zomeName}',
        fn_name: 'create_${snakeCase(type.name)}',
        payload: ${camelCase(type.name)},
        provenance: cellData.cell_id[1]
      });`,
    };
    methods['isCreateEnabled'] = {
      async: false,
      imports: [],
      params: [],
      fnContent: `return ${Object.values(type.fields)
        .map(f => `this.${camelCase(f.name)}`)
        .join(' && \n      ')};`,
    };
  }

  return {
    inject: [appWebsocketContext],
    template,
    localState,
    imports,

    methods,
  };
}

export function createTemplate(type: TypeDefinition<any, any>): ElementContent[] {
  if (!type.fields) return [];

  const fields: Element[] = type.fields.map(f => ({
    type: 'element',
    tagName: f.type.create[0].tagName,
    events: {
      change: `on${titleCase(f.name)}Change($event)`,
    },
    children: [],
  }));

  const createButton: Element = {
    type: 'element',
    tagName: 'mwc-button',
    inputs: {
      disabled: '!isCreateEnabled()',
    },
    events: {
      click: `create${titleCase(type.name)}()`,
    },
    children: [],
  };

  return [
    {
      type: 'element',
      tagName: 'div',
      properties: {
        style: 'display: flex; flex-direction: column',
      },
      children: [...fields, createButton],
    },
  ];
}
