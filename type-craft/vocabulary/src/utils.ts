import camelCase from 'lodash-es/camelCase';
import uniq from 'lodash-es/uniq';
import { FieldDefinition, Vocabulary, TypeDefinition } from './type-definition';

export function defaultSample(vocabulary: Vocabulary, fields: Array<FieldDefinition<any>>): any {
  const obj: Record<string, any> = {};

  for (const field of fields) {
    obj[camelCase(field.name)] = vocabulary[field.type].sample();
  }

  return obj;
}

export function getAllChildrenTypes(vocabulary: Vocabulary, type: TypeDefinition<any, any>): string[] {
  let childrenTypes: string[] = [type.name];

  if (type.fields) {
    for (const field of type.fields) {
      const granchildren = getAllChildrenTypes(vocabulary, vocabulary[field.type]);

      childrenTypes = [...childrenTypes, ...granchildren];
    }
  }

  return uniq(childrenTypes);
}
