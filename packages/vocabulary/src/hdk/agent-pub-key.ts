import { TypeDefinition } from '@type-craft/vocabulary';
import { TypeElementsImportDeclarations } from '@type-craft/web-components';
import { TypescriptTypeGenerator } from '@type-craft/typescript';
import { RustTypeGenerator } from '@type-craft/rust';
import { fakeAgentPubKey, serializeHash } from './utils';

export const type: TypeDefinition<string, {}> = {
  name: 'AgentPubKey',
  description: 'The identifier of an Agent in Holochain',

  sample: () => serializeHash(fakeAgentPubKey()),
};

export const tsGenerator: TypescriptTypeGenerator = {
  imports: [],
  defineType: '',
  referenceType: 'string',
};

export function rustGenerator(hdkVersion: string): RustTypeGenerator {
  return {
    imports: [
      {
        crateName: 'hdk',
        importDeclaration: `use hdk::prelude::holo_hash::AgentPubKeyB64;`,
        version: hdkVersion,
      },
    ],
    defineType: '',
    referenceType: 'AgentPubKeyB64',
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
