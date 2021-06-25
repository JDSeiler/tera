import { Result, Ok, Err } from './types.ts';

/**
 * What command the user provided
 */
export enum Command {
    Init,
    Help,
    TakeQuiz
}

/**
 * A user command and its arguments (may be empty)
 */
export interface Args {
    command: Command,
    arguments: string[]
}

/**
 * What kind of problem occured during Arg parsing
 */
export enum ArgErrorType {
    NoArgs,
    UnkownCommand,
    MissingParameter
}

/**
 * An argument error type and accompanying error message
 */
export interface ArgError {
    type: ArgErrorType,
    message: string
}

/**
 * TODO - parses the user arguments, may or may not make more fancy
 * @returns the arguments provided by the user, or an arg error
 */
export function parse(): Result<Args, ArgError> {
    if (Deno.args.length < 1) {
        return Err({
            type: ArgErrorType.NoArgs,
            message: 'No arguments were provided'
        });
    }

    const rawCommand = Deno.args[0];
    let parsedCmd;
    if (rawCommand === 'init') {
        parsedCmd = Command.Init;
    } else if (rawCommand === '--help') {
        parsedCmd = Command.Help;
    } else if (rawCommand === 'take') {
        parsedCmd = Command.TakeQuiz
        if (Deno.args.length < 2) {
            return Err({
                type: ArgErrorType.MissingParameter,
                message: `Command 'take' missing required parameter`
            });
        }
    } else {
        return Err({
            type: ArgErrorType.UnkownCommand,
            message: `${rawCommand} is not a known command`
        });
    }

    return Ok({
        command: parsedCmd,
        arguments: Deno.args.length >= 2 ? Deno.args.slice(1) : []
    })
}
