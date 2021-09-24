const path = require('path');

module.exports = {
  lintOnSave: false,
  outputDir: './dist',

  devServer: {
    overlay: {
      warnings: false,
      errors: true,
    },
  },
  configureWebpack: {
    resolve: {
      alias: {
        'lit-element': path.resolve('../../node_modules/lit-element'),
        'prismjs/prism.js': path.resolve('../../node_modules/prismjs/prism.js'),
        '/node_modules/prismjs/components': path.resolve('../../node_modules/prismjs/components'),
        '/node_modules/prismjs/themes/prism.css': path.resolve('../../node_modules/prismjs/themes/prism.css'),
        'lit-html': path.resolve('../../node_modules/lit-html'),
      },
    },
  },
  chainWebpack: config => {
    config.module
      .rule('vue')
      .use('vue-loader')
      .tap(options => ({
        ...options,
        compilerOptions: {
          // treat any tag that starts with ion- as custom elements
          isCustomElement: tag =>
            tag.startsWith('copyable-') || tag.startsWith('mwc-') || tag.startsWith('ui5-') || tag.startsWith('code-'),
        },
      }));
  },
};
