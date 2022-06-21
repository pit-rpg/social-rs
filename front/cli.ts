import {program} from 'commander';
import {runBuild} from './webpack_cli';

program
    .option('--prod', 'production build', false)
    .option('--dev', 'start webpack dev server', false)
    .option('--hot', 'enable webpack dev server hot reload', false)
    .action(options => {
        runBuild({
            isDevServer: options.dev,
            isProd: options.prod,
            isHot: options.hot
        });
    })
    .parse();
