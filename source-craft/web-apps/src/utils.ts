import { NpmImport } from '@source-craft/npm';
import ts from 'typescript';
import { NpmOrLocalImport, WebComponent } from './types';

export function allNpmImports(component: WebComponent): NpmImport[] {
  return allImports(component).filter(isNpmImport);
}

function isNpmImport(i: NpmOrLocalImport): i is NpmImport {
  return !!(i as NpmImport).importDeclaration;
}

export function allImportDeclarations(component: WebComponent): ts.ImportDeclaration[] {
  const allI = allImports(component);

  const allImportDeclarations = allI.filter(i => !isNpmImport(i)) as ts.ImportDeclaration[];

  const npmImportsDeclarations = allI.filter(i => isNpmImport(i)).map(i => (i as NpmImport).importDeclaration);

  return [...allImportDeclarations, ...npmImportsDeclarations];
}

function allImports(component: WebComponent): NpmOrLocalImport[] {
  let imports: NpmOrLocalImport[] = [];
  if (component.imports) {
    imports = imports.concat(component.imports);
  }
  if (component.inject) {
    for (const i of component.inject) {
      imports = imports.concat(i.imports);
    }
  }
  if (component.methods) {
    for (const i of Object.values(component.methods)) {
      imports = imports.concat(i.imports);
    }
  }
  if (component.onMounted) {
    imports = imports.concat(component.onMounted.imports);
  }

  return imports;
}
