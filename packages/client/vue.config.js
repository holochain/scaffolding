const path = require('path');

module.exports = {
  lintOnSave: false,
  outputDir: './dist',
  productionSourceMap: false,

  filenameHashing: false,
  devServer: {
    overlay: {
      warnings: false,
      errors: true,
    },
  },
  configureWebpack: {
    resolve: {
      alias: {
        '@material/mwc-notched-outline': path.resolve('../../node_modules/@material/mwc-notched-outline'),
        '@material/mwc-ripple': path.resolve('../../node_modules/@material/mwc-ripple'),
        '@material/mwc-list': path.resolve('../../node_modules/@material/mwc-list'),
        '@material/mwc-icon': path.resolve('../../node_modules/@material/mwc-icon'),
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
