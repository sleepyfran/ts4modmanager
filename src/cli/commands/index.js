import Help from './help';
import Install from './install';
import List from './list';
import Uninstall from './uninstall';
import Update from './update';
import Version from './version';

/**
 * Defines a command of the CLI.
 * @typedef {Object} Command
 * @property {string} name Name of the command.
 * @property {string} explanation Explanation of the command usage.
 * @property {string[]} matchingNames List of names that can reference the command. Should be unique.
 * @property {Function} execute Method that will execute the command.
 */
export default [Help, Install, List, Uninstall, Update, Version];
