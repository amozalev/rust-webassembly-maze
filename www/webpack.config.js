const path = require("path");
const MiniCssExtractPlugin = require("mini-css-extract-plugin");

const CopyWebpackPlugin = require("copy-webpack-plugin");

module.exports = {

    entry: "./bootstrap.js",
    module: {
        rules: [
            {
                test: /\.tsx?$/,
                use: 'ts-loader',
                exclude: /node_modules/,
            },
            {
                test: /\.css$/i,
                use: [MiniCssExtractPlugin.loader, 'css-loader']
            }
        ],
    },
    resolve: {
        extensions: ['.tsx', '.ts', '.js'],
    },
    output: {
        path: path.resolve(__dirname, "public"),
        filename: "bootstrap.js"
    },
    mode: "development",
    plugins: [
        new MiniCssExtractPlugin(),
        new CopyWebpackPlugin({
            patterns: [
                {from: "./index.html", to: "./"},
                { from: 'style.css', to: './' }
            ]
        }),
    ]
}