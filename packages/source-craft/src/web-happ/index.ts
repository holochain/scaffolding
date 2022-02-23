import { HappDefinition } from '@holochain-scaffolding/definitions';

import { happ } from '../happ';
import { PatcherDirectory } from '@source-craft/types';

import { webHappYaml } from './web-happ.yaml';
import { webApp, WebFramework } from '../web';

export function webHapp(happDef: HappDefinition, webFramework: WebFramework): PatcherDirectory {
  const happDir = happ(happDef);

  (happDir.children['workdir'] as PatcherDirectory).children['web-happ.yaml'] = webHappYaml({
    happName: happDef.name,
    uiBundlePath: '../ui/dist.zip',
    happBundlePath: `./${happDef.name}.happ`,
  });

  happDir.children['ui'] = webApp(happDef, webFramework);

  return happDir;
}
