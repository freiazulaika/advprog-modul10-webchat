const path = require('path');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');

const distPath = path.resolve(__dirname, 'dist');

module.exports = {
    mode: 'production',
    entry: './bootstrap.js',
    output: {
        path: distPath,
        filename: 'yewchat.js',
        webassemblyModuleFilename: 'yewchat_bg.wasm',
        clean: true,
    },
    devServer: {
        static: distPath,
        port: 8000,
    },
    experiments: {
        asyncWebAssembly: true,
    },
    module: {
        rules: [
            {
                test: /\.wasm$/,
                type: "webassembly/async",
            },
        ],
    },
    plugins: [
        new CopyWebpackPlugin({
            patterns: [{ from: 'static', to: '.' }],
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, '.'),
            extraArgs: '-- --features wee_alloc',
            outName: 'yewchat',
        }),
    ],
};
