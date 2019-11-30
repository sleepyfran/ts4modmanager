import chalk from 'chalk';

export default {
    name: 'update',
    explanation:
        'Updates all the currently installed mods to the latest version',
    matchingNames: ['u', 'update'],
    execute: args => {
        console.log(chalk.red('Not available yet'));
    },
};
