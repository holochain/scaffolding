import { ScDirectory, ScNodeType } from '@source-craft/types';
import { ContextProvided } from '@source-craft/web-apps';
import { addVueComponent, setAppComponent } from './component';
import camelCase from 'lodash-es/camelCase';
import upperFirst from 'lodash-es/upperFirst';
import { importDeclaration } from './utils';

/**
 *
 * @param dir
 * @param contextKey
 * @param contextType
 * @param fn is of the form: 'export function connect() {}'
 * @returns
 */
export function provideContextForApp(dir: ScDirectory, contextProvided: ContextProvided): ScDirectory {
  const srcDir = dir.children['src'] as ScDirectory;

  if (!srcDir.children['components']) {
    srcDir.children['components'] = {
      type: ScNodeType.Directory,
      children: {},
    };
  }
  if (!srcDir.children['contexts']) {
    srcDir.children['contexts'] = {
      type: ScNodeType.Directory,
      children: {},
    };
  }

  const contextIdentifier = camelCase(contextProvided.context.name);
  const contextFileName = `${contextIdentifier}.ts`;
  const createFnName = `create${upperFirst(contextIdentifier)}`;

  (srcDir.children['contexts'] as ScDirectory).children[contextFileName] = {
    type: ScNodeType.File,
    content: `${[...contextProvided.context.imports, ...contextProvided.createContext.imports].join('\n')}

export async function ${createFnName}(): Promise<${contextProvided.context.type}> {
${contextProvided.createContext.fnContent}
}
`,
  };

  // Create HelloWorld.vue

  dir = addVueComponent(dir, 'HelloWorld', {
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

  // Create App.vue

  dir = setAppComponent(dir, {
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
      importDeclaration(`import { computed } from 'vue';`),
      importDeclaration(`import HelloWorld from './components/HelloWorld.vue';`),
      importDeclaration(`import { ${createFnName} } from './contexts/${contextIdentifier}';`),
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

  return dir;
}
