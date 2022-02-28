export type Constructor<T> = new (...args: any[]) => T;

export type CreateElement<T, C> = {
  [key in keyof C]: C[key];
} & {
  fieldName: string;
  
  get value(): T;
};

export type DetailElement<T, C> = {
  [key in keyof C]: C[key];
} & {
  fieldName: string;
  
  value: T;
};
