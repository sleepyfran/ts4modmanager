import Commands, { Command } from './commands';
import NoOp from './commands/noop';
import { tail, drop } from 'lodash';

/**
 * Parses the given arguments and returns a matching command with the rest of the arguments that should be given to
 * the command execution method.
 * @param {string[]} args Arguments of the app.
 * @returns {[Command, string[]]} Array with the first element being the command and the second element being the arguments.
 */
export const parse = args => {
    if (!args) return [NoOp, []];
    if (args.length < 1) return [NoOp, []];

    const appArgs = drop(args, 2);
    const commandName = appArgs[0];
    const commandArgs = tail(appArgs);

    if (!commandName) return [NoOp, []];

    const matchingCommand = Commands.find(command =>
        command.matchingNames.includes(commandName),
    );

    return matchingCommand
        ? [matchingCommand, commandArgs]
        : [NoOp, commandArgs];
};
