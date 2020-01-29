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
        chalk.blue(
            `✅ Found ${modInfo.files.length} files to download for the mod \"${modInfo.name}\", which one do you want to download?`,
        ),
    );

    const numberOfFiles = modInfo.files.length;
    modInfo.files.forEach((file, index) => {
        console.log(chalk.green(`${index + 1}. ${file.name}`));
    });
    console.log(chalk.green(`${numberOfFiles + 1}. All of them`));

    return getInput(
        chalk.blue('🔢 Input the number from the file you wish to download'),
    ).then(input => {
        return +input === numberOfFiles + 1
            ? modInfo.files
            : modInfo.files[+input - 1]
            ? [modInfo.files[+input - 1]]
            : [];
    });
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
        console.log(chalk.blue(`🔍 Attempting to download mod from ${url}`));

        const [result, downloader] = findDownloader(url);
        if (result === NotAnUrl)
            return console.log(chalk.red('🚫 The given URL is not valid'));
        else if (result === NotAvailable)
            return console.log(chalk.red('👎 The given URL is not available'));
        else if (result === UnrecognizedSite)
            return console.log(chalk.red('☹️ The given URL is not supported'));

        console.log(
            chalk.blue(
                `🚚 The given URL was recognized for the downloader ${downloader.name}; fetching and parsing content`,
            ),
        );

        parseUrlContent(url, downloader)
            .then(modInfo => {
                if (modInfo.files.length > 1)
                    return multipleFilesChoice(modInfo);

                console.log(
                    chalk.blue(`✅ Found one file for the mod ${modInfo.name}`),
                );
                return modInfo.files;
            })
            .then(files => {
                if (files.length === 0)
                    return console.log(
                        chalk.red('⛔️ No files specified; exiting'),
                    );

                console.log(
                    chalk.blue(
                        `⬇️ Attempting to download ${files.length} files`,
                    ),
                );
            })
            .catch(err => {
                console.log(
                    chalk.red(
                        `🚫 There was an error retrieving the content, maybe the given URL is not a valid mod?`,
                    ),
                );
            });
    },
};
