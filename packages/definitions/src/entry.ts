import { FieldDefinition, TypeDefinition, defaultSample } from '@type-craft/vocabulary';

import snakeCase from 'lodash-es/snakeCase';
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
): RustTypeGenerator {
  const defineType = `#[hdk_entry(id = "${snakeCase(typeName)}")]
#[serde(rename_all = "camelCase")]
${defaultDefineType(happRustGenerators, typeName, fields)}`;

  return {
    imports: [
      {
        crateName: 'hdk',
        importDeclaration: 'use hdk::prelude::*;',
        version: '0.0.122',
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
