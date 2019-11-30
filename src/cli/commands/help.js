import Commands from './index';
import chalk from 'chalk';

export default {
    name: 'help',
    explanation: 'Shows a list of all the available commands',
    matchingNames: ['h', 'help'],
    execute: args => {
        console.log('These are the available commands in the app:');
        Commands.forEach(command => {
            console.log(
                `${chalk.green(command.name)} - ${command.explanation}`,
            );
        });
    },
};
