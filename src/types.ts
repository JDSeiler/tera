/*
Basically a shameless rip of Rust's Result type.
I could've written this in Rust instead but I wanted
to use Deno.
*/

export function Ok<T,E>(iValue: T): Result<T,E> {
    return {
        ok: true,
        value: iValue
    }
}

export function Err<T,E>(iErr: E): Result<T,E> {
    return {
        ok: false,
        value: iErr
    }
}

export function unwrap<T,E>(r: Result<T,E>): Option<T> {
    if (r.ok) {
        return r.value;
    } else {
        return null;
    }
}

export function unwrapErr<T,E>(r: Result<T,E>): Option<E> {
    if (!r.ok) {
        return r.value;
    } else {
        return null;
    }
}

// Credit: https://imhoff.blog/posts/using-results-in-typescript
// Futzed about with these types for way too long before settling on this
export type Result<T, E> = 
    {ok: true, value: T} | {ok: false, value: E}
export type Option<T> = T | null;