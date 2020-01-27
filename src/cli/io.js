import readLine from 'readline';

/**
 * Shows a prompt to retrieve the user's input.
 * @param {string?} message An optional message to show to the user.
 * @returns {Promise<string>} A promise that will contain the user's input.
 */
export const getInput = (message = null) => {
    const rl = readLine.createInterface({
        input: process.stdin,
        output: process.stdout,
    });

    return new Promise(resolve => {
        rl.question(`${message}: `, answer => {
            resolve(answer);
            rl.close();
        });
    });
};
