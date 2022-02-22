export type Constructor<T> = new (...args: any[]) => T;

export type CreateElement<T, C> = {
  [key in keyof C]: C[key];
} & {
  get value(): T;
};

export type DetailElement<T, C> = {
  [key in keyof C]: C[key];
} & {
  value: T;
};

export interface ElementReference<E> {
  element: Constructor<HTMLElement & E>;
  tagName: string;
  package: string;
  version: string;
  customImportDefiningCustomElement?: string;
}
