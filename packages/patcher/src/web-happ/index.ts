import { HappDefinition } from '@holochain/rad-definitions';

import { happ } from '../happ';
import { PatcherDirectory } from '@patcher/types';

import { webHappYaml } from './web-happ.yaml';
import { webApp, WebFramework } from '../web';

export async function webHapp(happDef: HappDefinition, webFramework: WebFramework): Promise<PatcherDirectory> {
  const happDir = await happ(happDef);

  (happDir.children['workdir'] as PatcherDirectory).children['web-happ.yaml'] = webHappYaml({
    happName: happDef.name,
    uiBundlePath: '../ui/dist.zip',
    happBundlePath: `./${happDef.name}.happ`,
  });

  happDir.children['ui'] = await webApp(happDef, webFramework);

  return happDir;
}
