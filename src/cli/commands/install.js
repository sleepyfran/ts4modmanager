import chalk from 'chalk';
import {
    findDownloader,
    NotAnUrl,
    NotAvailable,
    UnrecognizedSite,
} from '../../core/downloader';

export default {
    name: 'install',
    explanation: `Installs a new mod to the game. Usage: ${chalk.red(
        'install <url>',
    )}`,
    matchingNames: ['i', 'install'],
    execute: args => {
        if (args.length == 0) return console.log(chalk.red('No URL specified'));

        const url = args[0];
        console.log(chalk.blue(`Attempting to download mod from ${url} ...`));

        const [result, downloader] = findDownloader(url);
        if (result === NotAnUrl)
            return console.log(chalk.red('The given URL is not valid'));
        else if (result === NotAvailable)
            return console.log(chalk.red('The given URL is not available'));
        else if (result === UnrecognizedSite)
            return console.log(chalk.red('The given URL is not supported yet'));

        console.log(
            chalk.blue(
                `The given URL was recognized for the downloader ${downloader.name}; parsing content...`,
            ),
        );
    },
};
