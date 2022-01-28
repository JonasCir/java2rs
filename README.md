# java2rs

A Java to Rust transpiler written in Rust.

# How is it built?

Java parsing -> IR -> Rust code generation.

## Java parsing

Java parsing is done with [tree-sitter-java](https://github.com/tree-sitter/tree-sitter-java).

## IR

Small intermediate representation to run transformations on, e.g., grouping of static class methods.

## Rust codegen

This is done by using the [quote](https://github.com/dtolnay/quote) library which is well known in the context of Rust's
procedural macros. By using `quote`, we can quasi-quote the Rust code we want to
generate. [syn](https://github.com/dtolnay/syn) is used in case we need to run preprocessing for `quote`.
[prettyplease](https://github.com/dtolnay/prettyplease) is used for pretty printing the resulting Rust code.

# FAQ

## Can I use it in production?

No!

## Will it eat my laundry?

Yes!
