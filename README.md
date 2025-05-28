# Sudoku Checker

Check a Sudoku board's validity at compile time, using only the type system -
if the board is valid, it will compile, if it's not, it won't.

Note: this is just a PoC project, and was only made for fun (there isn't really
a reason for you to add this to your crate...).

## Example

```rust
use sudoku_checker::sudoku;

let sudoku = sudoku! {
    1, 2, _, _, _, _, _, _, _,
    _, _, _, _, _, _, _, _, _,
    _, 8, _, _, _, _, _, _, _,
    _, _, _, _, _, _, _, _, _,
    _, _, _, _, _, _, _, _, _,
    _, _, _, _, _, _, _, _, _,
    _, _, _, _, _, _, _, _, _,
    _, _, _, _, _, _, 7, _, _,
    _, _, _, _, _, _, _, _, 4,
};
```

## Motivation

I encountered this [post](https://www.reddit.com/r/rust/comments/1kvpdxz/sudoku_checker_in_rust_type_system),
which led me to this [cool project](https://github.com/gruhn/typescript-sudoku),
and I wanted to create something alike in Rust.
