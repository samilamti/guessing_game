If you'd like to learn Rust, [The Book](https://doc.rust-lang.org/book/) from Rust's official website, is freely available and quite good.

I've gotten to chapter 10ish and have adapted the book's guessing game to flex my newfound skills. Here's that game.

`secret_numbers.rs` defines the `SecretNumber` struct, representing a guessable secret number and Accuracy, representing how arbitrarilly close to the guess a given number is.

`main.rs` is the game itself, providing IO functionality and some randomness to make the game more interesting.
- `prompt` asks the user a question, ensuring that the answer is between a given range of values.
- `random_prompt_string` generates a semi-random question to ask.
- `main` is the game loop.