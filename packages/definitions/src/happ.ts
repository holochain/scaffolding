import { DnaDefinition } from './dna';

export interface HappDefinition {
  name: string;
  dnas: DnaDefinition[];
}

export interface WebHappDefinition {
  happ: HappDefinition;
  uiTemplate: string;
}
