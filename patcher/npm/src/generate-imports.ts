import { NpmImport } from './npm-import';

export function generateImports(imports: NpmImport[]): string {
  return `${imports
    .map(npmImport => `import {${npmImport.symbolsToImport.join(', ')}} from '${npmImport.packageName}'`)
    .join('\n')}`;
}
