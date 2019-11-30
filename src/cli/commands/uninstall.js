import chalk from 'chalk';

export default {
    name: 'uninstall',
    explanation: `Removes a mod from the game. Usage: ${chalk.red(
        'uninstall <name>',
    )}. If you need to find the name of a mod, use 'list'`,
    matchingNames: ['u', 'uninstall'],
    execute: args => {
        console.log(chalk.red('Not available yet'));
    },
};
