export default {
    name: 'noop',
    explanation: '',
    matchingNames: [''],
    execute: args => {
        console.log(
            "No input specified. Use 'help' to show the list of available commands.",
        );
    },
};
