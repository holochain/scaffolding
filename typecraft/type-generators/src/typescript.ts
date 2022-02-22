import { TypeDefinition, Vocabulary } from '@typecraft/type-definition';
import { NpmImport, generateImports } from '@patcher/npm';
import { PatcherFile, PatcherNodeType } from '@patcher/types';
import { upperFirst, camelCase, uniq, flattenDeep } from 'lodash-es';
import { TypeGenerators } from './types';
import { getAllChildrenTypes } from './utils';

export type TsTypeGenerators = TypeGenerators<TsTypeGenerator>;

export interface TsTypeGenerator {
  imports: NpmImport[];
  defineType: string;
  referenceType: string;
}

export function generateTsTypesFile(
  vocabulary: Vocabulary,
  typeGenerators: TsTypeGenerators,
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

function defaultGenerator(typeGeneratorsVocabulary: TsTypeGenerators, type: TypeDefinition<any, any>): TsTypeGenerator {
  return {
    imports: [],
    defineType: defaultDefineType(typeGeneratorsVocabulary, type),
    referenceType: upperFirst(camelCase(type.name)),
  };
}

function defaultDefineType(typeGeneratorsVocabulary: TsTypeGenerators, type: TypeDefinition<any, any>): string {
  return `export interface ${upperFirst(camelCase(type.name))} {
  ${
    type.fields
      ? type.fields.map(f => `${camelCase(f.name)}: ${typeGeneratorsVocabulary[f.type].referenceType};`).join('\n  ')
      : ''
  }
}`;
}
