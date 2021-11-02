const path = require('path');
const CopyPlugin = require('copy-webpack-plugin');
const webpack = require('webpack');

module.exports = {
  entry: './src/index.ts',
  target: 'node',
  mode: 'production',
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/,
      },
      {
        test: /\.m?js$/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['@babel/preset-env'],
          },
        },
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
          from: `${require.resolve('@holochain/scaffolding-ui')}/dist`,
          to: './public/',
        },
      ],
    }),
    new webpack.BannerPlugin({ banner: '#!/usr/bin/env node', raw: true }),
  ],
};
