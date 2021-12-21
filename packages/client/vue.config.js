const path = require('path');
const webpack = require('webpack');

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
    plugins: [
      new webpack.DefinePlugin({
        globalThis: 'window',
      }),
    ],
  },
  chainWebpack: config => {
    config.module
      .rule('vue')
      .use('vue-loader')
      .tap(options => ({
        ...options,
        compilerOptions: {
          // treat any tag that starts with ion- as custom elements
          isCustomElement: tag => tag.includes('-'),
        },
      }));
  },
};
