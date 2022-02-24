import { generateImports } from '@source-craft/cargo';
import { ScFile, ScNodeType } from '@source-craft/types';
import { getAllChildrenTypes, Vocabulary, TypeDefinition, FieldDefinition } from '@type-craft/vocabulary';
import { uniq, flattenDeep, upperFirst, camelCase, snakeCase } from 'lodash-es';
import { VocabularyRustGenerators, RustTypeGenerator } from './types';

export function generateRustTypesFile(
  vocabulary: Vocabulary,
  typeGenerators: VocabularyRustGenerators,
  types: Array<TypeDefinition<any, any>>,
): ScFile {
  const allChildrenTypeNames = uniq(flattenDeep(types.map(t => getAllChildrenTypes(vocabulary, t))));

  const generators = allChildrenTypeNames.map(t => {
    let g = typeGenerators[t];

    if (!g) g = defaultGenerator(typeGenerators, vocabulary[t].name, vocabulary[t].fields || []);
    return g;
  });

  const allImports = generators.map(g => g.imports);
  const imports = generateImports(flattenDeep(allImports));

  const allDefineTypes = generators.map(g => g.defineType).join('\n\n');

  return {
    type: ScNodeType.File,
    content: `${imports}

${allDefineTypes}`,
  };
}

export function defaultGenerator(
  typeGenerators: VocabularyRustGenerators,
  name: string,
  fields: Array<FieldDefinition<any>>,
): RustTypeGenerator {
  return {
    imports: [],
    defineType: defaultDefineType(typeGenerators, name, fields),
    referenceType: upperFirst(camelCase(name)),
  };
}

export function defaultDefineType(
  typeGenerators: VocabularyRustGenerators,
  name: string,
  fields: Array<FieldDefinition<any>>,
): string {
  return `#[derive(Clone)]
pub struct ${upperFirst(camelCase(name))} {
  pub ${fields.map(f => `${snakeCase(f.name)}: ${typeGenerators[f.type].referenceType},`).join('\n  ')}
}`;
}
