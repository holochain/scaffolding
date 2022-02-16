import { DnaDefinition, newDnaDef } from './dna';

export interface HappDefinition {
  name: string;
  dnas: DnaDefinition[];
}

export interface WebHappDefinition {
  happ: HappDefinition;
  uiTemplate: string;
}

export function newHappDef(): HappDefinition {
  return {
    name: 'my-app',
    dnas: [newDnaDef()],
  };
}
