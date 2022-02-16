import flattenDeep from 'lodash-es/flattenDeep';
import camelCase from 'lodash-es/camelCase';
import upperFirst from 'lodash-es/upperFirst';
import { HappDefinition } from '@holochain-scaffolding/definitions';

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

export const kebabToCamelCase = (s: string) => s.replace(/-./g, x => x.toUpperCase()[1]);
export const snakeToCamelCase = (s: string) => s.replace(/_./g, x => x.toUpperCase()[1]);

export function titleCase(str: string) {
  return upperFirst(camelCase(str));
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
