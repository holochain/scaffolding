export interface CargoImport {
  crateName: string;
  version: string;

  importDeclaration: string;
}

export function generateImports(imports: CargoImport[]): string {
  return `${imports.map(cargoImport => cargoImport.importDeclaration).join('\n')}`;
}
