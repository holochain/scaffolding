import { FieldDefinition, ProgrammingLanguages, TypeDefinition, TypeGenerator } from './index';
import camelCase from 'lodash-es/camelCase';
import upperFirst from 'lodash-es/upperFirst';
import snakeCase from 'lodash-es/snakeCase';
import uniq from 'lodash-es/uniq';
import flattenDeep from 'lodash-es/flattenDeep';

export function aggregateGenerators(
  type: TypeDefinition<any, any>,
  language: ProgrammingLanguages,
): { imports: string[]; defineTypes: string[] } {
  const generator = type.generators[language];

  const imports: string[] = [...generator.imports];
  const defineTypes: string[] = [generator.defineType];

  if (type.fields) {
    for (const field of type.fields) {
      imports.push(...field.type.generators[language].imports);
      defineTypes.push(field.type.generators[language].defineType);
    }
  }

  return {
    imports: uniq(flattenDeep(imports)),
    defineTypes: flattenDeep(defineTypes),
  };
}

export function generateTypesFile(types: Array<TypeDefinition<any, any>>, language: ProgrammingLanguages): string {
  const g = types.map(t => aggregateGenerators(t, language));

  const imports = g.map(a => a.imports);
  const defineTypes = g.map(a => a.defineTypes);

  return `${uniq(flattenDeep(imports)).join('\n')}

${flattenDeep(defineTypes).join('\n\n')}
`;
}

export function defaultTsGeneratorDefineType(typeName: string, fields: Array<FieldDefinition<any>>): string {
  return `export interface ${upperFirst(camelCase(typeName))} {
  ${fields
    .map(f => `${camelCase(f.name)}: ${f.type.generators[ProgrammingLanguages.Typescript].referenceType};`)
    .join('\n  ')}
}`;
}

export function defaultRustGeneratorDefineType(typeName: string, fields: Array<FieldDefinition<any>>): string {
  return `#[derive(Clone)]
pub struct ${upperFirst(camelCase(typeName))} {
  ${fields
    .map(f => `${snakeCase(f.name)}: ${f.type.generators[ProgrammingLanguages.Rust].referenceType},`)
    .join('\n  ')}
}`;
}

export function defaultTsGenerator(name: string, fields: Array<FieldDefinition<any>>): TypeGenerator {
  return {
    imports: [],
    defineType: defaultTsGeneratorDefineType(name, fields!),
    referenceType: name,
  };
}

export function defaultRustGenerator(name: string, fields: Array<FieldDefinition<any>>): TypeGenerator {
  return {
    imports: [],
    defineType: defaultRustGeneratorDefineType(name, fields!),
    referenceType: name,
  };
}

export function defaultSample(fields: Array<FieldDefinition<any>>): any {
  const obj: Record<string, any> = {};

  for (const field of fields) {
    obj[camelCase(field.name)] = field.type.sample();
  }

  return obj;
}
