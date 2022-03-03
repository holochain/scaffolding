import { TypeDefinition } from '@type-craft/vocabulary';
import { TypeElementsImportDeclarations } from '@type-craft/web-components';
import { TypescriptTypeGenerator } from '@type-craft/typescript';
import { RustTypeGenerator } from '@type-craft/rust';

export const type: TypeDefinition<string, {}> = {
  name: 'EntryHash',
  description: 'A hash of a Holochain entry',

  sample: () => 'uhCEkr6pGIyV6_lr2MbT_Siw0DXZInPa0cgA9B9Sq1NtokBr0IiM2',
};

export const tsGenerator: TypescriptTypeGenerator = {
  imports: [],
  defineType: 'export type EntryHashB64 = string;',
  referenceType: 'EntryHashB64',
};

export function rustGenerator(hdkVersion: string): RustTypeGenerator {
  return {
    imports: [
      {
        crateName: 'hdk',
        importDeclaration: `use hdk::prelude::holo_hash::EntryHashB64;`,
        version: hdkVersion,
      },
    ],
    defineType: '',
    referenceType: 'EntryHashB64',
  };
}

export const elementsImports: TypeElementsImportDeclarations = {
  detail: {
    sideEffectImport: {
      importDeclaration: `import '@holochain-open-dev/utils/copiable-hash';`,
      packageName: '@holochain-open-dev/utils',
      version: '^0.0.1',
    },
    tagName: 'copiable-hash',
  },
};
