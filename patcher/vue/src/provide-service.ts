import { PatcherDirectory, PatcherNodeType } from '@patcher/types';
import { ServiceProvided } from '@patcher/web-apps';
import { vueComponent } from './component';
import camelCase from 'lodash-es/camelCase';

/**
 *
 * @param dir
 * @param serviceKey
 * @param serviceType
 * @param fn is of the form: 'export function connect() {}'
 * @returns
 */
export function provideServiceForApp(dir: PatcherDirectory, serviceProvided: ServiceProvided): PatcherDirectory {
  const srcDir = dir.children['src'] as PatcherDirectory;

  if (!srcDir.children['components']) {
    srcDir.children['components'] = {
      type: PatcherNodeType.Directory,
      children: {},
    };
  }
  if (!srcDir.children['services']) {
    srcDir.children['services'] = {
      type: PatcherNodeType.Directory,
      children: {},
    };
  }

  const serviceIdentifier = camelCase(serviceProvided.service.name);
  const serviceFileName = `${serviceIdentifier}.ts`;
  const createFnName = `create${serviceIdentifier}`;

  (srcDir.children['services'] as PatcherDirectory).children[serviceFileName] = {
    type: PatcherNodeType.File,
    content: `${serviceProvided.imports.join('\n')}

export async function ${createFnName}(): Promise<${serviceProvided.service.type}> {
${serviceProvided.createFnContent}
}
`,
  };

  // Create HelloWorld.vue

  const helloWorld = vueComponent(
    {
      template: {
        tag: 'span',
        inner: [`{{ ${serviceIdentifier} }}`],
      },
      inject: [{ name: serviceIdentifier, type: serviceProvided.service.type }],
      imports: serviceProvided.imports,
    },
    `{ ${serviceIdentifier}: ${serviceProvided.service.type} }`,
  );

  (srcDir.children['components'] as PatcherDirectory).children['HelloWorld.vue'] = helloWorld;

  // Create App.vue

  const appVue = vueComponent({
    template: {
      tag: 'HelloWorld',
      ifCondition: serviceIdentifier,
    },
    localState: {
      [serviceIdentifier]: {
        default: undefined,
        type: `${serviceProvided.service.type} | undefined`,
      },
    },
    provide: [serviceProvided],
    imports: [
      `import { computed } from 'vue';`,
      `import HelloWorld from './components/HelloWorld.vue';`,
      `import { ${createFnName} } from './services/${serviceIdentifier}';`,
      ...serviceProvided.imports,
    ],
    subcomponents: ['HelloWorld'],
    onMounted: {
      async: true,
      callback: {
        params: [],
        fnContent: `this.${serviceIdentifier} = await ${createFnName}();`,
      },
    },
  });

  srcDir.children['App.vue'] = appVue;

  return dir;
}
