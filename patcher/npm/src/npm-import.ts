export interface NpmImport {
  packageName: string;
  version: string;
  symbolsToImport: string[];
  import: string;
}
