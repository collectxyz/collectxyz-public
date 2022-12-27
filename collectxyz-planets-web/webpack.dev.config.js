const webpack = require('webpack')
const path = require('path')
const { WebpackManifestPlugin } = require('webpack-manifest-plugin')

module.exports = {
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
  stats: {
    children: false,
  },
  mode: 'development',
  devtool: 'eval-source-map',
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
  devServer: {
    hot: true,
    port: '3001',
    proxy: {
      '*': {
        target: 'http://localhost:3000/',
        secure: false,
      },
    },
  },
  output: {
    path: path.join(__dirname, 'public'),
    filename: '[name].[hash].js',
    publicPath: '/public/',
  },
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
        test: /\.(eot|otf|gif|webp|ttf|woff|woff2|png|jpe?g)(\?.*)?$/,
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
