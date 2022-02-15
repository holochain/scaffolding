import { JSONSchema7, JSONSchema7Definition } from 'json-schema';

export interface TypeConfigSchema<C> extends JSONSchema7 {
  properties?:
    | {
        [key in keyof Partial<C>]: JSONSchema7Definition;
      }
    | undefined;
}

export type Constructor<T> = new (...args: any[]) => T;

export interface CreateElement<T, C> {
  configuration: C;

  get value(): T;
}

export interface DetailElement<T, C> {
  configuration: C;
  value: T;
}

export interface FieldDefinition<C> {
  name: string;
  type: TypeDefinition<any, any>;
  configuration: C;
}

export interface ElementReference<E> {
  element: Constructor<HTMLElement & E>;
  package: string;
  version: string;
  customImportDefiningCustomElement?: string;
}

export interface TypeDefinition<T, C> {
  name: string;
  description: string;

  fields?: Array<FieldDefinition<any>>;

  configurationSchema?: TypeConfigSchema<C>;
  create: Array<ElementReference<CreateElement<T, C>>>;
  detail: Array<ElementReference<DetailElement<T, C>>>;
}
