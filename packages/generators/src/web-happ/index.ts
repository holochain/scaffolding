import { HappDefinition } from '@holochain-scaffolding/definitions';

import { happ } from '../happ';
import { ScDirectory } from '@source-craft/types';

import { webHappYaml } from './web-happ.yaml';

export function webHapp(happDef: HappDefinition, webApp: ScDirectory): ScDirectory {
  const happDir = happ(happDef);

  (happDir.children['workdir'] as ScDirectory).children['web-happ.yaml'] = webHappYaml({
    happName: happDef.name,
    uiBundlePath: '../ui/dist.zip',
    happBundlePath: `./${happDef.name}.happ`,
  });

  happDir.children['ui'] = webApp;

  return happDir;
}
