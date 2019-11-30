import chalk from 'chalk';

export default {
    name: 'install',
    explanation: `Installs a new mod to the game. Usage: ${chalk.red(
        'install <url>',
    )}`,
    matchingNames: ['i', 'install'],
    execute: args => {
        console.log(chalk.red('Not available yet'));
    },
};
