import camelCase from 'lodash-es/camelCase';
import { FieldDefinition, Vocabulary } from './type-definition';

export function defaultSample(vocabulary: Vocabulary, fields: Array<FieldDefinition<any>>): any {
  const obj: Record<string, any> = {};

  for (const field of fields) {
    obj[camelCase(field.name)] = vocabulary[field.type].sample();
  }

  return obj;
}
