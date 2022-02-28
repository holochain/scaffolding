import { TypescriptTypeGenerator } from '@type-craft/typescript';
import { RustTypeGenerator } from '@type-craft/rust';

export const tsGenerator: TypescriptTypeGenerator = {
  imports: [],
  defineType: '',
  referenceType: 'string',
};

export const rustGenerator: RustTypeGenerator = {
  imports: [],
  defineType: '',
  referenceType: 'String',
};
