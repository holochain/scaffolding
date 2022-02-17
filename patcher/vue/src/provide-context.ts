import { PatcherDirectory, PatcherNodeType } from '@patcher/types';
import { ContextProvided } from '@patcher/web-apps';
import { vueComponent } from './component';
import camelCase from 'lodash-es/camelCase';
import upperFirst from 'lodash-es/upperFirst';

/**
 *
 * @param dir
 * @param contextKey
 * @param contextType
 * @param fn is of the form: 'export function connect() {}'
 * @returns
 */
export function provideContextForApp(dir: PatcherDirectory, contextProvided: ContextProvided): PatcherDirectory {
  const srcDir = dir.children['src'] as PatcherDirectory;

  if (!srcDir.children['components']) {
    srcDir.children['components'] = {
      type: PatcherNodeType.Directory,
      children: {},
    };
  }
  if (!srcDir.children['contexts']) {
    srcDir.children['contexts'] = {
      type: PatcherNodeType.Directory,
      children: {},
    };
  }

  const contextIdentifier = camelCase(contextProvided.context.name);
  const contextFileName = `${contextIdentifier}.ts`;
  const createFnName = `create${upperFirst(contextIdentifier)}`;

  (srcDir.children['contexts'] as PatcherDirectory).children[contextFileName] = {
    type: PatcherNodeType.File,
    content: `${[...contextProvided.context.imports, ...contextProvided.createContext.imports].join('\n')}

export async function ${createFnName}(): Promise<${contextProvided.context.type}> {
${contextProvided.createContext.fnContent}
}
`,
  };

  // Create HelloWorld.vue

  const helloWorld = vueComponent({
    template: [
      {
        type: 'element',
        tagName: 'span',
        children: [
          {
            type: 'text',
            value: `{{ ${contextIdentifier} }}`,
          },
        ],
      },
    ],
    inject: [contextProvided.context],
  });

  (srcDir.children['components'] as PatcherDirectory).children['HelloWorld.vue'] = helloWorld;

  // Create App.vue

  const appVue = vueComponent({
    template: [
      {
        type: 'ifCondition',
        condition: contextIdentifier,
        then: {
          type: 'element',
          tagName: 'HelloWorld',
          children: [],
        },
      },
    ],
    localState: {
      [contextIdentifier]: {
        default: undefined,
        type: `${contextProvided.context.type} | undefined`,
      },
    },
    provide: [contextProvided],
    imports: [
      `import { computed } from 'vue';`,
      `import HelloWorld from './components/HelloWorld.vue';`,
      `import { ${createFnName} } from './contexts/${contextIdentifier}';`,
      ...contextProvided.context.imports,
    ],
    subcomponents: ['HelloWorld'],
    onMounted: {
      async: true,
      params: [],
      fnContent: `this.${contextIdentifier} = await ${createFnName}();`,
      imports: [],
    },
  });

  srcDir.children['App.vue'] = appVue;

  return dir;
}
