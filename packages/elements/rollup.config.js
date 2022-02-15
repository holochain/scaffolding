import vue from 'rollup-plugin-vue';
import typescript from 'rollup-plugin-typescript2';

const pkg = require('./package.json');

export default {
  input: 'src/index.ts',
  output: {
    format: 'esm',
    dir: 'dist',
  },
  watch: {
    clearScreen: false,
  },
  external: [...Object.keys(pkg.dependencies), /@ui5\/webcomponents/],
  plugins: [
    typescript({}),
    vue({
      css: false,
      compilerOptions: {
        // treat any tag that starts with ion- as custom elements
        isCustomElement: tag => tag.includes('-'),
      },
    }),
  ],
};
