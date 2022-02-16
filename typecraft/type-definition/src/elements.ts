export type Constructor<T> = new (...args: any[]) => T;

export interface CreateElement<T, C> {
  // Needs to be a property of the element
  configuration: C;

  get value(): T;
}

export interface DetailElement<T, C> {
  configuration: C;
  value: T;
}

export interface ElementReference<E> {
  element: Constructor<HTMLElement & E>;
  package: string;
  version: string;
  customImportDefiningCustomElement?: string;
}