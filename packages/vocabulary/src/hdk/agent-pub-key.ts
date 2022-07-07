import { TypeDefinition } from '@type-craft/vocabulary';
import { TypeElementsImportDeclarations } from '@type-craft/web-components';
import { TypescriptTypeGenerator } from '@type-craft/typescript';
import { RustTypeGenerator } from '@type-craft/rust';
import { AgentPubKey } from '@holochain/client';
import { fakeAgentPubKey } from './utils';

export const type: TypeDefinition<AgentPubKey, {}> = {
  name: 'AgentPubKey',
  description: 'The identifier of an Agent in Holochain',

  sample: () => fakeAgentPubKey(),
};

export const tsGenerator: TypescriptTypeGenerator = {
  imports: [],
  defineType: '',
  referenceType: 'string',
};

export function rustGenerator(hdiVersion: string): RustTypeGenerator {
  return {
    imports: [
      {
        crateName: 'holochain_deterministic_integrity',
        importDeclaration: `use holochain_deterministic_integrity::prelude::*;`,
        version: hdiVersion,
      },
    ],
    defineType: '',
    referenceType: 'AgentPubKey',
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
