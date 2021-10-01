import nodeResolve from '@rollup/plugin-node-resolve';
import typescript from '@rollup/plugin-typescript';
import commonjs from '@rollup/plugin-commonjs';
import babel from '@rollup/plugin-babel';
import html from '@web/rollup-plugin-html';
import { importMetaAssets } from '@web/rollup-plugin-import-meta-assets';
import { terser } from 'rollup-plugin-terser';
import { generateSW } from 'rollup-plugin-workbox';
import path from 'path';
import replace from '@rollup/plugin-replace';

const production = !process.env.ROLLUP_WATCH;

export default {
  input: 'index.html',
  output: {
    entryFileNames: '[hash].js',
    chunkFileNames: '[hash].js',
    assetFileNames: '[hash][extname]',
    format: 'es',
    dir: 'dist',
  },
  preserveEntrySignatures: false,

  watch: {
    clearScreen: false,
  },

  plugins: [
    replace({
      'process.env.HC_PORT': production
        ? 'undefined'
        : JSON.stringify(process.env.HC_PORT),
      delimiters: ['', ''],
    }),
    /** Enable using HTML as rollup entrypoint */
    html({
      minify: true,
      injectServiceWorker: true,
      serviceWorkerPath: 'dist/sw.js',
    }),
    /** Resolve bare module imports */
    nodeResolve(),
    typescript({}),
    commonjs({}),
    /** Minify JS */
    production && terser(),
    /** Bundle assets references via import.meta.url */
    importMetaAssets(),
    /** Compile JS to a lower language target */
    production &&
      babel({
        babelHelpers: 'bundled',
        presets: [
          [
            require.resolve('@babel/preset-env'),
            {
              targets: [
                'last 3 Chrome major versions',
                'last 3 Firefox major versions',
                'last 3 Edge major versions',
                'last 3 Safari major versions',
              ],
              modules: false,
              bugfixes: true,
            },
          ],
        ],
        plugins: [
          [
            require.resolve('babel-plugin-template-html-minifier'),
            {
              modules: {
                lit: ['html', { name: 'css', encapsulation: 'style' }],
              },
              failOnError: false,
              strictCSS: true,
              htmlMinifier: {
                collapseWhitespace: true,
                conservativeCollapse: true,
                removeComments: true,
                caseSensitive: true,
                minifyCSS: true,
              },
            },
          ],
        ],
      }),
    /** Create and inject a service worker */
    production &&
      generateSW({
        globIgnores: ['polyfills/*.js', 'nomodule-*.js'],
        navigateFallback: '/index.html',
        // where to output the generated sw
        swDest: path.join('dist', 'sw.js'),
        // directory to match patterns against to be precached
        globDirectory: path.join('dist'),
        // cache any html js and css by default
        globPatterns: ['**/*.{html,js,css,webmanifest}'],
        skipWaiting: true,
        clientsClaim: true,
        runtimeCaching: [
          { urlPattern: 'polyfills/*.js', handler: 'CacheFirst' },
        ],
      }),
  ],
};
