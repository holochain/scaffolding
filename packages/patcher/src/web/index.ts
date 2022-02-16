import { HappDefinition } from '@holochain-scaffolding/definitions';
import { PatcherDirectory, PatcherFile } from '@patcher/types';
import { generateVueApp, provideServiceForApp, patchEnvVars, patchNpmDependency } from '@patcher/vue';
import { generateTsTypesForHapp } from '../ts';
import { addWebComponentsForHapp } from './add-component';

export enum WebFramework {
  Vue = 'vue',
}

export function webApp(happDef: HappDefinition, framework: WebFramework): PatcherDirectory {
  if (framework === WebFramework.Vue) {
    const dir = generateVueApp();

    dir.children['package.json'] = patchNpmDependency(
      dir.children['package.json'] as PatcherFile,
      '@holochain/client',
      '^0.3.2',
    );

    provideServiceForApp(dir, {
      imports: [`import { AppWebsocket } from '@holochain/client';`],
      createFnContent: `return AppWebsocket.connect(\`ws://localhost:\${import.meta.env.VITE_HC_PORT}\`)`,
      service: {
        name: 'appWebsocket',
        type: 'AppWebsocket',
      },
    });

    patchEnvVars(dir, {
      start: {
        VITE_HC_PORT: '$HC_PORT',
      },
    });

    const src = dir.children['src'] as PatcherDirectory;
    if (src) {
      src.children['types'] = generateTsTypesForHapp(happDef);
      // For every entry, add create and detail component
      // TODO: add dependencies for the elements to package.json
      src.children['components'] = addWebComponentsForHapp(src.children['components'] as PatcherDirectory, happDef);
    }

    return dir;
  }
}
