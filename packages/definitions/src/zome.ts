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
  sample: any;
}
