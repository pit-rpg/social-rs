import {inspect} from 'util';
import webpack from 'webpack';
import WebpackDevServer from 'webpack-dev-server';

import {getConfig, ConfigOptions} from './config';

export async function runBuild(options: ConfigOptions) {
    if (options.isProd) {
        process.env.NODE_ENV = 'production';
    }

    console.log('Options:', inspect(options, {colors: true, compact: false}));

    const webpackConfig = getConfig(options);
    const compiler = webpack(webpackConfig);

    if (options.isDevServer) {
        const server = new WebpackDevServer(compiler as any, webpackConfig.devServer);
        await server.start();
    } else {
        compiler.run((err, stats) => {
            console.log(stats?.toString({
                colors: true,
            }));

            if (err || stats?.hasErrors()) {
                console.error(err?.stack || err);
                if ((err as any)?.details) {
                    console.error((err as any).details);
                }

                process.exit(1);
            }
        });
    }
}