import {
    findDownloader,
    parseUrlContent,
    NotAnUrl,
    NotAvailable,
    UnrecognizedSite,
} from '../../core/downloader';
import { ModInfo, FileInfo } from '../../core/page-parser';
import { getInput } from '../io';
import chalk from 'chalk';

/**
 * Shows a choice to download one or various files to the user.
 * @param {ModInfo} modInfo Parsed information of the mod to display.
 * @returns {Promise<FileInfo[]>} Promise with the files to download.
 */
const multipleFilesChoice = async modInfo => {
    console.log(
        chalk.green(
            `Found ${modInfo.files.length} files to download for the mod \"${modInfo.name}\", which one do you want to download?`,
        ),
    );

    modInfo.files.forEach((file, index) => {
        console.log(chalk.green(`${index + 1}. ${file.name}`));
    });

    return getInput('Input the number from the file you wish to download').then(
        input => {
            return modInfo.files[input];
        },
    );
};

export default {
    name: 'install',
    explanation: `Installs a new mod to the game. Usage: ${chalk.red(
        'install <url>',
    )}`,
    matchingNames: ['i', 'install'],
    execute: args => {
        if (args.length == 0) return console.log(chalk.red('No URL specified'));

        const url = args[0];
        console.log(chalk.blue(`Attempting to download mod from ${url}`));

        const [result, downloader] = findDownloader(url);
        if (result === NotAnUrl)
            return console.log(chalk.red('The given URL is not valid'));
        else if (result === NotAvailable)
            return console.log(chalk.red('The given URL is not available'));
        else if (result === UnrecognizedSite)
            return console.log(chalk.red('The given URL is not supported'));

        console.log(
            chalk.blue(
                `The given URL was recognized for the downloader ${downloader.name}; parsing content`,
            ),
        );

        parseUrlContent(url, downloader)
            .then(modInfo => {
                if (modInfo.files.length > 1)
                    return multipleFilesChoice(modInfo);
            })
            .then(files => console.log(files));
    },
};
