import { flattenDeep } from 'lodash-es';
import { HappDefinition } from '@holochain/rad-definitions';

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

export function mergeStrings(strings: Array<any>, separator = '') {
  return flattenDeep(strings).join(separator);
}

/** Case Utils */

export const camelize = (s: string) => kebabToCamelCase(snakeToCamelCase(s));

export const kebabToCamelCase = (s: string) => s.replace(/-./g, x => x.toUpperCase()[1]);
export const snakeToCamelCase = (s: string) => s.replace(/_./g, x => x.toUpperCase()[1]);

export const kebabToSnakeCase = (str: string) =>
  str.replace(/[A-Z]/g, letter => `_${letter.toLowerCase()}`).replace(/\-/g, letter => `_`);

export function toTitleCase(str: string) {
  return camelize(str).replace(/\w\S*/g, function(txt) {
    return txt.charAt(0).toUpperCase() + txt.substr(1);
  });
}

const snakeCaseRegex = /^([a-z]{1,})(_[a-z0-9]{1,})*$/;

/**
 * Check for valid snake case.
 * https://en.wikipedia.org/wiki/Snake_case
 *
 * Allowed characters are [a-z], [0-9] and _
 *
 * @param
 */
export function isSnakeCase(input: string) {
  return snakeCaseRegex.test(input);
}
