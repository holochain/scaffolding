import { HappDefinition } from '@holochain-scaffolding/definitions';
import { PatcherDirectory, PatcherFile } from '@patcher/types';
import { generateVueApp, provideServiceForApp, patchEnvVars, patchNpmDependency } from '@patcher/vue';
import { generateTsTypes } from '../ts';

export enum WebFramework {
  Vue = 'vue',
}

export async function webApp(happDef: HappDefinition, framework: WebFramework): Promise<PatcherDirectory> {
  if (framework === WebFramework.Vue) {
    const dir = generateVueApp();

    dir.children['package.json'] = patchNpmDependency(dir.children['package.json'] as PatcherFile, '@holochain/client', '^0.3.2');

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
      src.children['types.ts'] = await generateTsTypes(happDef);
    }

    return dir;
  }
}
