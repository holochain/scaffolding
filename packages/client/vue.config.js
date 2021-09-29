const path = require('path');

module.exports = {
  lintOnSave: false,
  outputDir: './dist',

  filenameHashing: false,
  devServer: {
    overlay: {
      warnings: false,
      errors: true,
    },
  },
  configureWebpack: {
    resolve: {
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
