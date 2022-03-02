import { TypeDefinition } from '@type-craft/vocabulary';
import { TypeElementsImportDeclarations } from '@type-craft/web-components';
import { TypescriptTypeGenerator } from '@type-craft/typescript';
import { RustTypeGenerator } from '@type-craft/rust';

export const type: TypeDefinition<string, {}> = {
  name: 'EntryHash',
  description: 'A hash of a Holochain entry',

  sample: () => 'uhCEkuRNJ_3yZw64zed1JwvMgjiAslcCqxfl7sk3tiZ6aLoM',
};

export const tsGenerator: TypescriptTypeGenerator = {
  imports: [],
  defineType: '',
  referenceType: 'string',
};

export const rustGenerator: RustTypeGenerator = {
  imports: [
    {
      crateName: 'hdk',
      importDeclaration: `use hdk::prelude::holo_hash::EntryHashB64;`,
      version: '0.0.122',
    },
  ],
  defineType: '',
  referenceType: 'EntryHashB64',
};

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
