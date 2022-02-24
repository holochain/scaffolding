import { TypescriptTypeGenerator } from '@type-craft/typescript';
import { RustTypeGenerator } from '@type-craft/rust';

export const tsGenerator: TypescriptTypeGenerator = {
  imports: [],
  defineType: '',
  referenceType: 'number',
};

export const rustGenerator: RustTypeGenerator = {
  imports: [],
  defineType: '',
  referenceType: 'usize',
};
