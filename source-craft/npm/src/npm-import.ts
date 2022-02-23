import * as ts from 'typescript';

export interface NpmImport {
  importDeclaration: ts.ImportDeclaration;

  packageName: string;
  version: string;
}
