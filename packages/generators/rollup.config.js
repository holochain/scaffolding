import typescript from '@rollup/plugin-typescript';
import resolve from '@rollup/plugin-node-resolve';
import commonjs from '@rollup/plugin-commonjs';

const pkg = require('./package.json');

export default {
  input: `src/index.ts`,
  output: { dir: 'dist', format: 'es', sourcemap: true },
  external: [...Object.keys(pkg.dependencies)],
  plugins: [typescript(), resolve(), commonjs()],
};
