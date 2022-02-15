import { TypeDefinition } from '@typecraft/type-definition';

export interface ZomeDefinition {
  name: string;
  entry_defs: EntryDefinition[];
}

export interface EntryDefinition {
  name: string;
  read: boolean;
  create: boolean;
  update: boolean;
  delete: boolean;

  typeDefinition: TypeDefinition<any, any>;
}


export const hdkTypes = ['AgentPubKeyB64', 'EntryHashB64', 'HeaderHashB64'];
