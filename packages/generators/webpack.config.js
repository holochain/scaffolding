const path = require('path');

module.exports = {
  entry: './src/index.ts',
  mode: 'development',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      { test: /\.hbs$/, loader: 'handlebars-loader' },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js', '.hbs'],
  },
  output: {
    globalObject: 'this',
    filename: 'index.js',
    path: path.resolve(__dirname, 'dist'),
    libraryTarget: 'umd',
  },
};
