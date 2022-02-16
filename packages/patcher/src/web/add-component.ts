import { EntryDefinition, HappDefinition } from '@holochain-scaffolding/definitions';
import { PatcherDirectory, PatcherFile, PatcherNodeType } from '@patcher/types';
import { FnDefinition, WebComponent } from '@patcher/web-apps';
import { vueComponent } from '@patcher/vue';
import { TypeDefinition } from '@typecraft/type-definition';
import { ElementContent, Element } from 'hast';
import camelCase from 'lodash-es/camelCase';
import { titleCase } from '../utils';
import snakeCase from 'lodash-es/snakeCase';

export function addWebComponentsForHapp(componentsDir: PatcherDirectory, happ: HappDefinition): PatcherDirectory {
  for (const dna of happ.dnas) {
    const dnaDir: PatcherDirectory = {
      type: PatcherNodeType.Directory,
      children: {},
    };

    for (const zome of dna.zomes) {
      const zomeDir: PatcherDirectory = {
        type: PatcherNodeType.Directory,
        children: {},
      };

      for (const entry of zome.entry_defs) {
        const dir = webComponentsForEntry(dna.name, zome.name, entry);
        zomeDir.children = {
          ...zomeDir.children,
          ...dir.children,
        };
      }

      dnaDir.children[zome.name] = zomeDir;
    }

    componentsDir.children[dna.name] = dnaDir;
  }

  return componentsDir;
}

export function webComponentsForEntry(dnaName: string, zomeName: string, entry: EntryDefinition): PatcherDirectory {
  const wcs: WebComponent[] = [createWebComponent(dnaName, zomeName, entry.typeDefinition)];

  const children: Record<string, PatcherFile> = {};
  for (const wc of wcs) {
    children[`Create${titleCase(entry.name)}.vue`] = vueComponent(wc);
  }

  return {
    type: PatcherNodeType.Directory,
    children,
  };
}

export function createWebComponent(dnaName: string, zomeName: string, type: TypeDefinition<any, any>): WebComponent {
  const template = createTemplate(type);

  const methods: Record<string, FnDefinition> = {};

  if (type.fields) {
    for (const field of type.fields) {
      methods[`on${titleCase(field.name)}Change`] = {
        params: [{ name: 'event', type: 'Event' }],
        fnContent: `this.${camelCase(type.name)}.${camelCase(field.name)} = (event.target as any).value;`,
      };
    }
    methods[`create${titleCase(type.name)}`] = {
      params: [],
      fnContent: `this.appWebsocket.callZome({
        cap_secret: null,
        cell_id: [], // TODO: fix
        zome_name: '${zomeName}',
        fn_name: 'create_${snakeCase(type.name)}',
        payload: this.${camelCase(type.name)}
      }`,
    };
  }

  return {
    inject: [
      {
        name: 'appWebsocket',
        type: 'AppWebsocket',
      },
    ],
    template,
    localState: {
      [camelCase(type.name)]: {
        type: titleCase(type.name),
        default: JSON.stringify(type.sample()),
      },
    },
    imports: [`import { ${titleCase(type.name)} } from '../../../types/${dnaName}/${zomeName}';`],

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
    events: {
      click: `create${titleCase(type.name)}`,
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
