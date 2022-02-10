import { HappDefinition } from '@holochain/rad-definitions';

import { happ } from '../happ';
import { PatcherDirectory } from '@patcher/types';

import { webHappYaml } from './web-happ.yaml';
import { generateTsTypes } from '../ts';

export async function webHapp(happDef: HappDefinition, uiDir: PatcherDirectory): Promise<PatcherDirectory> {
  const src = uiDir.children['src'] as PatcherDirectory;
  if (src) {
    src.children['types.ts'] = await generateTsTypes(happDef);
  }

  const happDir = await happ(happDef);

  (happDir.children['workdir'] as PatcherDirectory).children['web-happ.yaml'] = webHappYaml({
    happName: happDef.name,
    uiBundlePath: '../ui/dist.zip',
    happBundlePath: `./${happDef.name}.happ`,
  });

  happDir.children['ui'] = uiDir;

  return happDir;
}
