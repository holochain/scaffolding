import { TypeDefinition, Vocabulary } from '@typecraft/type-definition';
import { CargoImport, generateImports } from '@patcher/cargo';
import { TypeGenerators } from './types';
import { upperFirst, camelCase, snakeCase, flattenDeep, uniq } from 'lodash-es';
import { PatcherFile, PatcherNodeType } from '@patcher/types';
import { getAllChildrenTypes } from './utils';

export type RustTypeGenerators = TypeGenerators<RustTypeGenerator>;

export interface RustTypeGenerator {
  imports: CargoImport[];
  defineType: string;
  referenceType: string;
}

export function generateRustTypesFile(
  vocabulary: Vocabulary,
  typeGenerators: RustTypeGenerators,
  types: Array<TypeDefinition<any, any>>,
): PatcherFile {
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
    type: PatcherNodeType.File,
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
