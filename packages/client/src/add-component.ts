import { EntryDefinition, HappDefinition } from '@holochain-scaffolding/definitions';
import { PatcherDirectory, PatcherFile, PatcherNodeType } from '@patcher/types';
import { FnDefinition, WebComponent } from '@patcher/web-apps';
import { vueComponent } from '@patcher/vue';
import { TypeDefinition } from '@typecraft/type-definition';
import { ElementContent, Element } from 'hast';
import camelCase from 'lodash-es/camelCase';

export function addWebComponentsForHapp(dir: PatcherDirectory, happ: HappDefinition): PatcherDirectory {
  const componentsDir = (dir.children['src'] as PatcherDirectory).children['components'] as PatcherDirectory;

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
        const dir = webComponentsForEntry(entry);
        zomeDir.children = {
          ...zomeDir.children,
          ...dir.children,
        };
      }

      dnaDir.children[zome.name] = zomeDir;
    }

    componentsDir.children[dna.name] = dnaDir;
  }

  return dir;
}

const titleCase = (s: string) => `${s[0] && s[0].toUpperCase()}${camelCase(s.slice(1))}`;

export function webComponentsForEntry(entry: EntryDefinition): PatcherDirectory {
  const wcs: WebComponent[] = [createWebComponent(entry.typeDefinition)];

  const children: Record<string, PatcherFile> = {};
  for (const wc of wcs) {
    children[`Create${titleCase(entry.name)}.vue`] = vueComponent(wc);
  }

  return {
    type: PatcherNodeType.Directory,
    children,
  };
}

export function createWebComponent(type: TypeDefinition<any, any>): WebComponent {
  const template = createTemplate(type);

  const methods: Record<string, FnDefinition> = {};

  if (type.fields) {
    for (const field of type.fields) {
      methods[`on${field.name}Change`] = {
        params: [{ name: 'event', type: 'Event' }],
        fnContent: `this.${type.name}.${field.name} = event.target.value;`,
      };
    }
  }

  return {
    template,
    localState: {
      [camelCase(type.name)]: {
        type: type.name,
      },
    },

    methods,
  };
}

export function createTemplate(type: TypeDefinition<any, any>): ElementContent[] {
  if (!type.fields) return [];

  const fields: Element[] = type.fields.map(f => ({
    type: 'element',
    tagName: `create-${f.name}`,
    events: {
      change: `on${f.name}Change`,
    },
    children: [],
  }));

  const createButton: Element = {
    type: 'element',
    tagName: 'mwc-button',
    events: {
      click: `create${type.name}`,
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
