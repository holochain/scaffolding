const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');
const webpack = require('webpack');

module.exports = {
  entry: './src/app.ts',
  target: 'node',
  mode: 'development',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
    ],
  },
  resolve: {
    extensions: ['.tsx', '.ts', '.js'],
  },
  output: {
    globalObject: 'this',
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
    new webpack.BannerPlugin({ banner: '#!/usr/bin/env node', raw: true }),
  ],
};
