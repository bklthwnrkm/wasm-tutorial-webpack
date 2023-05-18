const HtmlWebpackPlugin = require("html-webpack-plugin");

module.exports = {
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true,
  },
  module: {
    rules: [{ test: /\.ts$/, loader: "ts-loader" }],
  },
  resolve: {
    extensions: [".js", ".ts", ".wasm"],
  },
  watchOptions: {
    ignored: "**/node_modules",
  },
  plugins: [new HtmlWebpackPlugin()],
};
