If you'd like to learn Rust, [The Book](https://doc.rust-lang.org/book/) from Rust's official website, is freely available and quite good.

I've gotten to chapter 10ish and have adapted the book's guessing game to flex my newfound skills. Here's that game.

`secret_numbers.rs` defines the `SecretNumber` struct, representing a guessable secret number and `Accuracy`, representing how arbitrarily close to the guess a given number is.

Some interesting points are the [derived traits automatically provided by the compiler](https://doc.rust-lang.org/stable/rust-by-example/trait/derive.html) by decorating the `Accuracy` enum with `#[derive(PartialEq, Eq, Hash)]`.
- `PartialEq` and `Eq` implements functionality to compare values and, well - if you don't have it (or your own implementation), the compuler will tell you
> error[E0277]: can't compare `secret_number::Accuracy` with `secret_number::Accuracy`
   --> src\secret_number.rs:12:10
    |
12  | #[derive(Eq, Hash, Debug)]
    |          ^^ no implementation for `secret_number::Accuracy == secret_number::Accuracy`
    |
    = help: the trait `std::cmp::PartialEq` is not implemented for `secret_number::Accuracy`

- `Hash` implements the supporting functionality to put values in a HashMap (a Dictionary).


`main.rs` is the game itself, providing IO functionality and some randomness to make the game more interesting.
- `prompt` asks the user a question, ensuring that the answer is between a given range of values.
- `random_prompt_string` generates a semi-random question to ask.
- `main` is the game loop.