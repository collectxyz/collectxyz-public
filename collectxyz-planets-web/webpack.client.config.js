const path = require('path')
const webpack = require('webpack')
const { WebpackManifestPlugin } = require('webpack-manifest-plugin')

module.exports = {
  mode: 'production',
  entry: {
    'client': [
      './src/client.tsx',
    ],
  },
  resolve: {
    extensions: ['.ts', '.tsx', '.js', '.jsx', '.json'],
    modules: [
      'src',
      'node_modules',
    ],
    alias: {
      src: path.resolve(__dirname, 'src'),
    },
    fallback: {
      'stream': require.resolve('stream-browserify'),
      'buffer': require.resolve('buffer'),
    },
  },
  output: {
    path: path.join(__dirname, 'public'),
    filename: '[name].[contenthash].js',
  },
  plugins: [
    new webpack.ProvidePlugin({
      Buffer: ['buffer', 'Buffer'],
      process: 'process/browser',
    }),
    new WebpackManifestPlugin({
      fileName: 'build-manifest.json',
      writeToFileEmit: true,
    }),
  ],
  module: {
    rules: [
      {
        test: /\.(ts|tsx)$/,
        use: [
          {
            loader: 'babel-loader',
          },
        ],
      },
      {
        test: /\.svg$/,
        use: [
          {
            loader: 'url-loader',
          },
        ],
      },
      {
        test: /\.(eot|otf|gif|webp|ttf|woff|woff2|png|jpeg)(\?.*)?$/,
        use: [
          {
            loader: 'file-loader',
            options: {
              name: '[name].[contenthash].[ext]',
            },
          },
        ],
      },
    ],
  },
}
