import { DnaDefinition, newDnaDef } from './dna';

export interface HappDefinition {
  name: string;
  dnas: DnaDefinition[];
}

export interface WebHappDefinition {
  happ: HappDefinition;
  uiTemplate: string;
}

export function newHappDef(happName = 'my-app'): HappDefinition {
  return {
    name: happName,
    dnas: [newDnaDef()],
  };
}
