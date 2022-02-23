import { NpmImport } from '@source-craft/npm';

export type TsTypeGenerators = Record<string, TsTypeGenerator>;

export interface TsTypeGenerator {
  imports: NpmImport[];
  defineType: string;
  referenceType: string;
}
