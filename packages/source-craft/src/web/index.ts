import { HappDefinition } from '@holochain-scaffolding/definitions';
import { PatcherDirectory, PatcherFile } from '@source-craft/types';
import { generateVueApp, provideContextForApp, patchEnvVars, patchNpmDependency } from '@source-craft/vue';
import { generateTsTypesForHapp } from '../ts';
import { addWebComponentsForHapp } from './components';
import { appWebsocketContext } from './appWebsocketContext';

export enum WebFramework {
  Vue = 'vue',
}

export function webApp(happDef: HappDefinition, framework: WebFramework): PatcherDirectory {
  if (framework === WebFramework.Vue) {
    let dir = generateVueApp();

    dir.children['package.json'] = patchNpmDependency(
      dir.children['package.json'] as PatcherFile,
      '@holochain/client',
      '^0.3.2',
    );
    dir.children['package.json'] = patchNpmDependency(
      dir.children['package.json'] as PatcherFile,
      '@types/ws',
      '^8.2.3',
    );

    provideContextForApp(dir, {
      createContext: {
        async: false,
        imports: [],
        fnContent: `const appWs = await AppWebsocket.connect(\`ws://localhost:\${import.meta.env.VITE_HC_PORT}\`);

        const appInfo = await appWs.appInfo({ installed_app_id: '${happDef.name}' });
        return {
          appInfo, 
          appWs
        };`,
        params: [],
      },
      context: appWebsocketContext,
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
    }
    dir = addWebComponentsForHapp(dir, happDef);

    return dir;
  }
}
