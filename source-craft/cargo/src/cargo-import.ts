export interface CargoImport {
  crateName: string;
  version: string;

  symbolsToImport: string[];
  import: string;
}

export function generateImports(imports: CargoImport[]): string {
  return `${imports
    .map(cargoImport => `use ${cargoImport.crateName}::${symbolsImport(cargoImport.symbolsToImport)};`)
    .join('\n')}`;
}

function symbolsImport(symbols: string[]) {
  if (symbols.length === 1) return symbols[0];
  else return `{${symbols.join(', ')}}`;
}
