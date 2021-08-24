const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');

module.exports = {
  entry: './src/app.ts',
  target: 'node',
  mode: 'production',
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
    filename: 'app.js',
    path: path.resolve(__dirname, 'dist'),
  },
  node: {
    global: false,
    __filename: false,
    __dirname: false,
  },
  plugins: [
    new CopyPlugin({
      patterns: [
        {
          from: '../../node_modules/@holochain/create-ui/dist',
          to: './public/',
        },
      ],
    }),
  ],
};
