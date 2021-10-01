module.exports = {
  lintOnSave: false,
  configureWebpack: {
    resolve: {
      alias: {
        '@material/mwc-ripple': path.resolve('./node_modules/@material/mwc-ripple'),
        '@material/mwc-button': path.resolve('./node_modules/@material/mwc-button'),
        '@material/mwc-icon': path.resolve('./node_modules/@material/mwc-icon'),
        '@material/mwc-dialog': path.resolve('./node_modules/@material/mwc-dialog'),
        '@material/mwc-snackbar': path.resolve('./node_modules/@material/mwc-snackbar'),
        '@material/mwc-notched-outline': path.resolve('./node_modules/@material/mwc-notched-outline'),
        '@material/mwc-icon-button': path.resolve('./node_modules/@material/mwc-icon-button'),
        '@material/mwc-textarea': path.resolve('./node_modules/@material/mwc-textarea'),
        '@material/mwc-textfield': path.resolve('./node_modules/@material/mwc-textfield'),
        '@material/mwc-fab': path.resolve('./node_modules/@material/mwc-fab'),
        '@material/mwc-circular-progress': path.resolve('./node_modules/@material/mwc-circular-progress'),
      },
    },
  },
};
