import { DnaDefinition, EntryDefinition, ZomeDefinition } from '@holochain/rad-definitions';

export function newEntryDef(name: string = 'entry_def_0'): EntryDefinition {
  return {
    create: true,
    read: true,
    delete: true,
    update: true,
    name,
    sample: {
      foo: 'hi',
      bar: 3
    },
  };
}

export function newZome(name: string = 'zome_0'): ZomeDefinition {
  return {
    entry_defs: [newEntryDef()],
    name,
  };
}

export function newDna(name: string = 'dna_0'): DnaDefinition {
  return {
    name,
    zomes: [newZome()],
  };
}
