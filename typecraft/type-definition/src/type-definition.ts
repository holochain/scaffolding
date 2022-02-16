import { ElementReference, CreateElement, DetailElement } from './elements';
import { JSONSchema7, JSONSchema7Definition } from 'json-schema';

export interface TypeConfigSchema<C> extends JSONSchema7 {
  properties?:
    | {
        [key in keyof Partial<C>]: JSONSchema7Definition;
      }
    | undefined;
}

export interface FieldDefinition<C> {
  name: string;
  type: TypeDefinition<any, any>;
  configuration: C;
}

export enum ProgrammingLanguages {
  Typescript,
  Rust,
}

export interface TypeGenerator {
  imports: string[];
  defineType: string;
  referenceType: string;
}

export type TypeGenerators = {
  [key in ProgrammingLanguages]: TypeGenerator;
};

export interface TypeDefinition<T, C> {
  name: string;
  description: string;

  fields?: Array<FieldDefinition<any>>;

  generators: TypeGenerators;
  sample: () => T;

  configurationSchema?: TypeConfigSchema<C>;
  create: Array<ElementReference<CreateElement<T, C>>>;
  detail: Array<ElementReference<DetailElement<T, C>>>;
}
