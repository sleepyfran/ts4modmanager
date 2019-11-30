import chalk from 'chalk';

export default {
    name: 'noop',
    explanation: '',
    matchingNames: [''],
    execute: () => {
        console.log(
            chalk.red(
                "No input specified. Use 'help' to show the list of available commands.",
            ),
        );
    },
};
