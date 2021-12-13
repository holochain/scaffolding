import { flattenDeep } from 'lodash-es';
import { HappDefinition } from '../types/happ';

export const camelize = (s: string) => kebabToCamelCase(snakeToCamelCase(s));

export const kebabToCamelCase = (s: string) => s.replace(/-./g, x => x.toUpperCase()[1]);
export const snakeToCamelCase = (s: string) => s.replace(/_./g, x => x.toUpperCase()[1]);

export const kebabToSnakeCase = (str: string) =>
  str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`).replace(/\-/g, letter => `_`);

export function getDnaBundlePath(happ: HappDefinition, dnaName: string): string {
  return `${getDnaPath(happ, dnaName)}workdir/${dnaName}.dna`;
}

export function getDnaPath(happ: HappDefinition, dnaName: string): string {
  if (happ.dnas.length === 1) return `dna/`;
  else return `dnas/${dnaName}/`;
}

export function getUiPackageName(happ: HappDefinition): string {
  return `ui`;
}

export function mergeStrings(strings: Array<any>) {
  return flattenDeep(strings).reduce((acc, next) => `${acc}${next}`, '');
}
