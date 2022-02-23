import { generateImports } from "@source-craft/npm";
import { ScFile, ScNodeType } from "@source-craft/types";
import { uniq, flattenDeep, upperFirst, camelCase } from "lodash-es";
import { getAllChildrenTypes, Vocabulary, TypeDefinition } from '@type-craft/vocabulary';

import { TsTypeGenerators, TsTypeGenerator } from "./types";

export function generateTsTypesFile(
  vocabulary: Vocabulary,
  typeGenerators: TsTypeGenerators,
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
