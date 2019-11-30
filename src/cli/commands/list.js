import chalk from 'chalk';

export default {
    name: 'list',
    explanation: 'Shows a list of all the installed mods',
    matchingNames: ['ls', 'list'],
    execute: args => {
        console.log(chalk.red('No mods installed yet'));
    },
};
