# uniffi-example

Project demonstrating use of https://github.com/mozilla/uniffi-rs.

This project is a workspace (but it doesn't have to be, it's that way simply to organize multiple projects), containing the following members:

- [simple](https://github.com/zduny/uniffi-example/tree/main/simple) - library we're trying to generate bindings to, 
it could be an external library, outside of this workspace, put here inside only for convenience.
- [vectors](https://github.com/zduny/uniffi-example/tree/main/vectors) - another, slightly more complicated library we're trying to generate bindings to, 
it could be an external library, outside of this workspace, put here inside only for convenience.
- [bindings-simple](https://github.com/zduny/uniffi-example/tree/main/bindings-simple) - intermediary layer wrapping 
[simple](https://github.com/zduny/uniffi-example/tree/main/simple) library, used as a base for generated bindings,
- [bindings-vectors](https://github.com/zduny/uniffi-example/tree/main/bindings-vectors) - intermediary layer wrapping 
[vectors](https://github.com/zduny/uniffi-example/tree/main/vectors) library, used as a base for generated bindings.

## Building

Simply call:

```bash
cargo build
```

## Generating bindings and testing

Use scripts in the root directory:

```bash
./test.sh
./generate.sh
```
