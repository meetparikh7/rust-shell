# Command parsing

Lets parse the command now and echo back the parsed command!

## Tokenizing

> A lexical token or simply token is a string with an assigned and thus identified meaning. It is structured as a pair consisting of a token name and an optional token value. The token name is a category of lexical unit.
>
> -- [<cite>Wikipedia</cite>](https://en.wikipedia.org/wiki/Lexical_analysis#Token)

Lets break down a command into a list of individual tokens.

Since we are coding quite a simple shell, we dont have token types, all of them are strings! Note we are not going to be entirely like bash and adopting a simplified approach.

Lets also not handle new lines and other escaped control characters except space and quotes

Lets look at some examples of what our tokenization should look like

| Example Description            | Command                               | Tokens                                       |
| ------------------------------ | ------------------------------------- | -------------------------------------------- |
| single token                   | `echo`                                | `echo`                                       |
| multiple tokens                | `echo a b`                            | `echo`, `a`, `b`                             |
| quotes                         | `echo "a b"`                          | `echo`, `a b`                                |
| multiple quotes                | `echo "a b" "c"`                     | `echo`, `a b`, `c`                           |
| escaped quotes                 | `echo "a \" b" \"x\"`                 | `echo`, `a " b`, `"x"`                       |
| escaped spaces                 | `echo a\ b`                           | `echo`, `a b`                                |
| complex                        | `NAME="<Your name>" echo Hello $NAME` | `NAME=<Your name>`, `echo`, `Hello`, `$NAME` |

## Parser module

We want to keep out code neat. So lets split the parser module. Splitting in Rust is very easy:

Add this to `main.rs`

```
mod parser;
```

This defines that there is a as of now undefined module parser. You could have defined it here using `mod parser { <code here> }`. However if you leave it like this, Rust will try to find a file `<module_name>.rs`. You will see how to define the module in the same file below

So let's write `parser.rs`

```
pub fn parse(s: &str) -> Option<Vec<String>> {
    Some(vec![String::from(s)])
}
```

This defines a public function in the module which parses a string and returns a vector of these string tokens

You will have noticed that a lot of stuff ends with `!`. These are macros in Rust, which help us use syntax sugar which can otherwise get cumbersome to write. `vec![*]` constructs a new vector from the elements listed.

And now use it in `main.rs` by changing the line which printed the command to

```
// Print the parsed command
println!("You entered: {:?}", parser::parse(command.trim()));
```

## Statements and expressions

Most things in rust are expressions and not statements. And if you write an expression at the end (without a `;`) it would be returned.

This explains how `vec![s]` is equivalent to `return vec![s]` since it is the last expression of the function.

This also works for `if` statements, which serve as ternary operators and if/else blocks.


## A note about the `&` and ownership

You will have noticed a lot of places using `&`, `&mut`, etc. I cant do this section justice and would recommend reading the [ownership section](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html) from the book for this. This is really what sets rust apart! So I do recommend reading it

## `Option`?

An `Option<T>` is a type which is either `Some(T)` or `None`. We will return `None` when the input is erroneous or else a `Vec` of `String`

## Unit tests

Now that we have documented what we want, lets go ahead and do some [BDD](https://en.wikipedia.org/wiki/Behavior-driven_development) and write some unit tests!

Add the following to the `parser.rs`.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tokenization() {
        assert_eq!(parse("echo"), Some(vec!["echo"]));
    }
}
```

Note that the `tests` module is not public and hence cannot be used by `main`. (A public module is defined using `pub mod`)

`unwrap` is a helper method in `Option` which causes a `panic!` if the type is `None` or else returns the

Run `cargo test` to see this test pass!

Now go on and add the other tests. Note that you will have to escape `\` and `"` with additional `\`

## The shlex crate

This kind of tokenization is provided by the `shlex` module in Python. What if we could get this utility in Rust!

So lets visit [crates.io](https://crates.io) and search for `shlex`. And voila, we find the [shlex crate](https://crates.io/crates/shlex).

Adding this to our project, is as easy as copying the line from the page to `Cargo.toml` in the `[dependencies]` section

Now lets use this in parser. This is the final source code for `parser.rs`

```rust
{{#include ../src/parser.rs}}
```
