import { fromRollup } from '@web/dev-server-rollup';
import rollupReplace from '@rollup/plugin-replace';

const replace = fromRollup(rollupReplace);

export default /** @type {import('@web/dev-server').DevServerConfig} */ ({
  open: true,
  /** Resolve bare module imports */
  nodeResolve: {
    exportConditions: ['browser', 'development'],
  },

  watch: true,

  rootDir: 'public',

  /** Compile JS for older browsers. Requires @web/dev-server-esbuild plugin */
  // esbuildTarget: 'auto'

  /** Set appIndex to enable SPA routing */
  appIndex: 'index.html',
  clearTerminalOnReload: false,
  plugins: [
    replace({
      'process.env.HC_PORT': JSON.stringify(process.env.HC_PORT),
      delimiters: ['', ''],
    }),

    /** Use Hot Module Replacement by uncommenting. Requires @open-wc/dev-server-hmr plugin */
    // hmr && hmrPlugin({ exclude: ['**/*/node_modules/**/*'], presets: [presets.litElement] }),
  ],

  // See documentation for all available options
});
