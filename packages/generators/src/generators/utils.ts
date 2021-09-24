import { HappDefinition } from '../types';

export const camelToSnakeCase = (str: string) =>
  str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`).replace(/\-/g, letter => `_`);

export function getDnaBundlePath(happ: HappDefinition, dnaName: string): string {
  return `${getDnaPath(happ, dnaName)}workdir/${dnaName}.dna`;
}

export function getDnaPath(happ: HappDefinition, dnaName: string): string {
  if (happ.dnas.length === 1) return `dna/`;
  else return `dnas/${dnaName}/`;
}

export function getUiPackageName(happ: HappDefinition): string {
  return `${happ.name}-ui`;
}
