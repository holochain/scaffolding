import {
  defaultRustGeneratorDefineType,
  defaultSample,
  defaultTsGeneratorDefineType,
  FieldDefinition,
  ProgrammingLanguages,
  TypeDefinition,
  TypeGenerator,
} from '@typecraft/type-definition';
import { dateType } from '@typecraft/date';
import snakeCase from 'lodash-es/snakeCase';
import camelCase from 'lodash-es/camelCase';
import upperFirst from 'lodash-es/upperFirst';

export interface EntryDefinition {
  read: boolean;
  create: boolean;
  update: boolean;
  delete: boolean;

  typeDefinition: TypeDefinition<any, any>;
}

export function newEntryDef(name: string = 'entry_def_0'): EntryDefinition {
  const fields = [
    {
      name: 'new_field',
      configuration: {},
      type: dateType,
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

export function holochainEntryRustTypeGenerator(typeName: string, fields: Array<FieldDefinition<any>>): TypeGenerator {
  const imports = ['use hdk::prelude::*;'];
  const defineType = `#[hdk_entry(id = "${snakeCase(typeName)}")]
#[serde(rename_all = "camelCase")]
${defaultRustGeneratorDefineType(typeName, fields)}`;

  return {
    imports,
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
    description: `Holochain entry containing a ${name}`,
    fields,
    create: [],
    detail: [],

    sample: () => defaultSample(fields),

    generators: {
      [ProgrammingLanguages.Typescript]: {
        imports: [],
        defineType: defaultTsGeneratorDefineType(name, fields),
        referenceType: name,
      },
      [ProgrammingLanguages.Rust]: holochainEntryRustTypeGenerator(name, fields),
    },
  };
}
