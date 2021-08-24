module.exports = {
  lintOnSave: false,
  outputDir: './dist',

  devServer: {
    overlay: {
      warnings: false,
      errors: true,
    },
  },

  transpileDependencies: ['vuetify'],
};
