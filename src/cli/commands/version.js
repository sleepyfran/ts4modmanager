import { version } from '../../../package.json';
import chalk from 'chalk';

export default {
    name: 'version',
    explanation: 'Shows the version of TS4 Mod Manager',
    matchingNames: ['v', 'version'],
    execute: args => {
        console.log(chalk.green(`TS4 Mod Manager v${version}`));
        console.log(
            chalk.blue(
                'Created by @sleepyfran (https://github.com/sleepyfran)',
            ),
        );
    },
};
