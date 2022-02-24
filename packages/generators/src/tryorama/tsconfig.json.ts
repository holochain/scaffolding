import { ScFile, ScNodeType } from '@source-craft/types';

export const tryoramaTsConfig = (): ScFile => ({
  type: ScNodeType.File,
  content: `{
  "compilerOptions": {
    /* Visit https://aka.ms/tsconfig.json to read more about this file */
    /* Basic Options */
    "outDir": "dist",
    "target": "ES2017",
    "module": "ESNext",
    "moduleResolution": "node",
    /* Strict Type-Checking Options */
    "strict": true /* Enable all strict type-checking options. */,
    "esModuleInterop": true /* Enables emit interoperability between CommonJS and ES Modules via creation of namespace objects for all imports. Implies 'allowSyntheticDefaultImports'. */,
    "noImplicitAny": false /* Raise error on expressions and declarations with an implied 'any' type. */,
    /* Advanced Options */
    "skipLibCheck": true /* Skip type checking of declaration files. */,
    "forceConsistentCasingInFileNames": true /* Disallow inconsistently-cased references to the same file. */
  }
}
`,
});
