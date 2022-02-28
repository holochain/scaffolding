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