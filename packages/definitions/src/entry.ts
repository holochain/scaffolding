import { FieldDefinition, TypeDefinition, defaultSample } from '@type-craft/vocabulary';

import camelCase from 'lodash-es/camelCase';
import upperFirst from 'lodash-es/upperFirst';
import { RustTypeGenerator, defaultDefineType } from '@type-craft/rust';
import { happRustGenerators, happVocabulary } from '@holochain-scaffolding/vocabulary';

export interface EntryDefinition {
  read: boolean;
  create: boolean;
  update: boolean;
  delete: boolean;

  typeDefinition: TypeDefinition<any, any>;
}

export function newEntryDef(name = 'entry_def_0'): EntryDefinition {
  const fields: Array<FieldDefinition<any>> = [
    {
      name: 'title',
      configuration: {},
      type: 'Title',
    },
    {
      name: 'content',
      configuration: {},
      type: 'Content',
    },
  ];
  return {
    create: true,
    read: true,
    delete: true,
    update: true,

    typeDefinition: holochainEntryTypeDefinition(name, fields),
  };
}

export function holochainEntryRustTypeGenerator(
  typeName: string,
  fields: Array<FieldDefinition<any>>,
  hdiVersion: string,
): RustTypeGenerator {
  const defineType = `#[hdk_entry_helper]
${defaultDefineType(happRustGenerators(hdiVersion), typeName, fields)}`;

  return {
    imports: [
      {
        crateName: 'hdi',
        importDeclaration: 'use hdi::prelude::*;',
        version: hdiVersion,
      },
    ],
    defineType,
    referenceType: upperFirst(camelCase(typeName)),
  };
}

export function holochainEntryTypeDefinition(
  name: string,
  fields: Array<FieldDefinition<any>>,
): TypeDefinition<any, any> {
  return {
    name,
    description: `Holochain entry for a ${name}`,
    fields,

    sample: () => defaultSample(happVocabulary, fields),
  };
}
