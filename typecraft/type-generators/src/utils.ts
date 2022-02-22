import { Vocabulary, TypeDefinition } from '@typecraft/type-definition';

export function getAllChildrenTypes(vocabulary: Vocabulary, type: TypeDefinition<any, any>): string[] {
  let childrenTypes: string[] = [];

  for (const field of type.fields) {
    const granchildren = getAllChildrenTypes(vocabulary, vocabulary[field.type]);

    childrenTypes = [...childrenTypes, ...granchildren];
  }

  return childrenTypes;
}
