import { NpmImport } from '@source-craft/npm';

export type VocabularyElementsImports = Record<string, TypeElementsImports>;

export interface CustomElementImport {
  sideEffectImport: NpmImport;
  tagName: string;
}

export interface TypeElementsImports {
  create: CustomElementImport;
  detail: CustomElementImport;
}

export function getAllImports(renderersImports: TypeElementsImports): NpmImport[] {
  return [renderersImports.create.sideEffectImport, renderersImports.detail.sideEffectImport];
}
