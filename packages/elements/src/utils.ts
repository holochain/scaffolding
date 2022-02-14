import { DnaDefinition, EntryDefinition, HappDefinition, ZomeDefinition } from '@holochain-scaffolding/definitions';

export function newEntryDef(name: string = 'entry_def_0'): EntryDefinition {
  return {
    create: true,
    read: true,
    delete: true,
    update: true,
    name,
    sample: {
      foo: 'hi',
      bar: 3,
    },
  };
}

export function newZomeDef(name: string = 'zome_0'): ZomeDefinition {
  return {
    entry_defs: [newEntryDef()],
    name,
  };
}

export function newDnaDef(name: string = 'dna_0'): DnaDefinition {
  return {
    name,
    zomes: [newZomeDef()],
  };
}

export function newHappDef(): HappDefinition {
  return {
    name: 'my-app',
    dnas: [newDnaDef()],
  };
}
