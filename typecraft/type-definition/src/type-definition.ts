import { CreateElement, DetailElement, Constructor } from './elements';
import { JSONSchema7, JSONSchema7Definition } from 'json-schema';

export interface ConfigurationSchema<C> extends JSONSchema7 {
  properties?:
    | {
        [key in keyof Partial<C>]: JSONSchema7Definition;
      }
    | undefined;
}

export interface FieldDefinition<C> {
  name: string;
  type: string;
  configuration: C;
}

export interface TypeDefinition<T, C> {
  name: string;
  description: string;

  fields?: Array<FieldDefinition<any>>;

  sample: () => T;

  configurationSchema?: ConfigurationSchema<C>;
}

export type Vocabulary = { [key: string]: TypeDefinition<any, any> };
/* 
export type Renderers<V extends Vocabulary> = { [key in keyof Partial<V>]: () => string };

export type AgentPubKey = string;

export const agentPubKey: TypeDefinition<AgentPubKey, Record<string, never>> = {
  name: 'AgentPubKey',
  description: '',
  sample: () => 'ujkkasjflksajlfkjsalkfjsaljfsdf',
};

export const header: TypeDefinition<any, any> = {
  name: 'header',
  description: '',
  fields: [
    {
      name: 'author',
      type: agentPubKey.name,
      configuration: {},
    },
  ],
  sample: () => ({ author: 'ulksjalfjslkfjalsjf' }),
};

const v: Vocabulary = {
  AgentPubKey: agentPubKey,
  Header: header,
};

const codeGen = {
  AgentPubKey: agentPubKey,
  Header: header,
};

const r: Renderers<v> = {
  AgentPubKey: (pubkey: string) => '<span>',
  Header: () => '<span>',
};
 */
