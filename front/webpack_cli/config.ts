import path from 'path';
import webpack from 'webpack';
import webpackDevServer from 'webpack-dev-server';
import HtmlWebpackPlugin from 'html-webpack-plugin';
// import { getTransformer } from 'ts-transform-graphql-tag';

export type WebPackConf = webpack.Configuration & {devServer: webpackDevServer.Configuration};

export type ConfigOptions = {
    isDevServer: boolean;
    isProd: boolean;
    isHot: boolean;
}

const DIR_DIST_NAME = 'dist';
const DIR_ASSETS_NAME = 'assets';
const DIR_APP_NAME = 'app';
const DIR_ROOT = path.resolve(path.dirname(__filename), '../../');
const DIR_DIST = path.resolve(DIR_ROOT, DIR_DIST_NAME);
const DIR_FRONT = path.resolve(path.dirname(__filename), '../');
const DIR_APP = path.resolve(DIR_FRONT, DIR_APP_NAME);


export function getConfig (options: ConfigOptions) {
    const {isDevServer, isHot, isProd} = options;

    const config: WebPackConf = {
        mode: isProd ? 'production' : 'development',
        devtool: isProd ? false : 'source-map',
        entry: {
            app: `./${DIR_APP_NAME}`,
        },
        output: {
            filename: 'js/[name].js',
            path: DIR_DIST,
        },
        optimization: {
            removeAvailableModules: false,
            removeEmptyChunks: false,
            splitChunks: false,
        },
        module: {
            rules: [
                {
                    test: /\.tsx?$/,
                    use: {
                        // options: {
                        //     // ... other loader's options
                        //     getCustomTransformers: () => ({ before: [getTransformer()] })
                        // },
                        loader: `ts-loader`,
                    },
                },
                {
                    test: /\.(png|jpg|gif)$/i,
                    use: [
                        {
                            loader: 'url-loader',
                            options: {
                                fallback: 'responsive-loader',
                            },
                        },
                    ],
                },
                {
                    test: /\.less$/i,
                    use: [
                      "style-loader",
                      "css-loader",
                      "less-loader",
                    ],
                },
            ]
        },
        resolve: {
            extensions: ['.tsx', '.ts', '.js', '.json', '.wasm'],
            symlinks: false,
            alias: {
                'app': `${DIR_APP}`
            }
        },
        plugins: [
            new HtmlWebpackPlugin({
                template: `./${DIR_ASSETS_NAME}/index.html`,
                // title: '',
                inject: 'body',
                hash: true,
                publicPath: '/',
            }),
        ],
        context: DIR_FRONT,
        devServer: {
            hot: isHot,
            historyApiFallback: true,
            proxy: {
                '/graphql': {
                    target: 'http://localhost:3000',
                    ws: true,
                },
                // '/ws': {
                //     target: 'http://localhost:3000',
                //     ws: true,
                // },
            }
        }
    };

    return config;
}