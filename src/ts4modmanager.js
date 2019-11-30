import { parse } from './cli/parser';

const args = process.argv;

const [command, commandArgs] = parse(args);
command.execute(commandArgs);
