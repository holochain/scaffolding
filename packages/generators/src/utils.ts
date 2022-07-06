import flattenDeep from 'lodash-es/flattenDeep';
import camelCase from 'lodash-es/camelCase';
import upperFirst from 'lodash-es/upperFirst';
import { HappDefinition } from '@holochain-scaffolding/definitions';
import { IntegrityZomeDefinition, CoordinatorZomeDefinition, ZomeBundleDefinition } from '@holochain-scaffolding/definitions';


export function getDnaBundlePath(happ: HappDefinition, dnaName: string): string {
  return `${getDnaPath(happ, dnaName)}workdir/${dnaName}.dna`;
}

export function getDnaPath(happ: HappDefinition, dnaName: string): string {
  if (happ.dnas.length === 1) return `dna/`;
  else return `dnas/${dnaName}/`;
}

export function getUiPackageName(_happ: HappDefinition): string {
  return `ui`;
}

export function mergeStrings(strings: Array<any>, separator = ''): string {
  return flattenDeep(strings).join(separator);
}

/** Case Utils */

export function titleCase(str: string): string {
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
export function isSnakeCase(input: string): boolean {
  return snakeCaseRegex.test(input);
}

export function zomeBundlesForIntegrityZomes(integrityZomes: IntegrityZomeDefinition[]): ZomeBundleDefinition[] {

  let zomeBundles: ZomeBundleDefinition[] = [];

  for (const integrityZome of integrityZomes) {
    const coordinatorZome: CoordinatorZomeDefinition = {
      name: integrityZome.name.slice(0, -10), // cutting off the "_integrity" part of the name
      dependencies: [integrityZome]
    }
    const zomeBundle: ZomeBundleDefinition = {
      name: integrityZome.name.slice(0, -10),
      integrityZome,
      coordinatorZome,
    }

    zomeBundles.push(zomeBundle);
  }

  return zomeBundles;
};


