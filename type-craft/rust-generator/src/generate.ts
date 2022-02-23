import { generateImports } from '@source-craft/cargo';
import { ScFile, ScNodeType } from '@source-craft/types';
import { getAllChildrenTypes, Vocabulary, TypeDefinition } from '@type-craft/vocabulary';
import { uniq, flattenDeep, upperFirst, camelCase, snakeCase } from 'lodash-es';
import { RustTypeGenerators, RustTypeGenerator } from './types';

export function generateRustTypesFile(
  vocabulary: Vocabulary,
  typeGenerators: RustTypeGenerators,
  types: Array<TypeDefinition<any, any>>,
): ScFile {
  const allChildrenTypeNames = uniq(flattenDeep(types.map(t => getAllChildrenTypes(vocabulary, t))));

  const generators = allChildrenTypeNames.map(t => {
    let g = typeGenerators[t];
    if (!g) g = defaultGenerator(typeGenerators, vocabulary[t]);
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

function defaultGenerator(
  typeGeneratorsVocabulary: RustTypeGenerators,
  type: TypeDefinition<any, any>,
): RustTypeGenerator {
  return {
    imports: [],
    defineType: defaultDefineType(typeGeneratorsVocabulary, type),
    referenceType: upperFirst(camelCase(type.name)),
  };
}

function defaultDefineType(typeGeneratorsVocabulary: RustTypeGenerators, type: TypeDefinition<any, any>): string {
  return `#[derive(Clone)]
pub struct ${upperFirst(camelCase(type.name))} {
  ${type.fields.map(f => `${snakeCase(f.name)}: ${typeGeneratorsVocabulary[f.type].referenceType},`).join('\n  ')}
}`;
}
