const path = require('path')
const nodeExternals = require('webpack-node-externals')

module.exports = {
  mode: 'production',
  externals: [nodeExternals()],
  target: 'node',
  entry: {
    'server': [
      './src/server',
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
  },
  output: {
    path: path.join(__dirname, 'private'),
    filename: '[name].js',
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
