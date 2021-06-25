import { parse } from './argParser.ts';
import { unwrap, unwrapErr } from './types.ts'; 

console.log('Welcome to Deno');
const maybeArgs = parse();
if (maybeArgs.ok) {
   const args = unwrap(maybeArgs);
   // Use a command dictionary approach, pass the array of args to the command.
} else {
   const err = unwrapErr(maybeArgs);
   console.log(err?.message);
}
