use std::io::{self, Write};
use rand::Rng;
use std::collections::HashMap;
use crate::guess::Guess;
use crate::guess::Accuracy;

mod guess;

fn random_number(from: u16, to_exclusive: u16) -> u16 {
    return rand::thread_rng().gen_range(from, to_exclusive);
}

fn prompt(question: String, secret_range_from: u16, secret_range_to_excl: u16) -> Guess {
    loop {
        print!("{}", question);
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse::<u16>() {
            Ok(num) => return Guess::new(num),
            Err(_) => {
                println!("Please guess a number between {} and {} ... ;)", secret_range_from, secret_range_to_excl);
                continue;
            },
        };
    }
}

fn random_prompt_string() -> String {
    // Let's go with a crazy one-liner, because we *can*!
    return ["Guess again: ", "You next guess? "][random_number(0, 2) as usize].to_string()
}

fn main() {
    println!("Guess the number! ");

    let secret_range_from = random_number(1, 101);
    let secret_range_multiplier = random_number(3, 6);
    let secret_range_to_excl = secret_range_from * secret_range_multiplier;
    let secret_number = random_number(secret_range_from, secret_range_to_excl);
    let mut number_of_guesses = 0;

    let mut accuracy_responses = HashMap::new();
    accuracy_responses.insert(Accuracy::More, "larger");
    accuracy_responses.insert(Accuracy::MoreThanDouble, "more than double");
    accuracy_responses.insert(Accuracy::MuchMore, "a lot larger");
    accuracy_responses.insert(Accuracy::Less, "smaller");
    accuracy_responses.insert(Accuracy::LessThanHalf, "less than half");
    accuracy_responses.insert(Accuracy::MuchLess, "a lot smaller");

    let mut guess = prompt(format!("Guess a number between {} and {}: ", secret_range_from, secret_range_to_excl), secret_range_from, secret_range_to_excl);
    loop {        
        number_of_guesses += 1;

        if guess.value() == 42 && secret_number != 42 {
            println!("42 is the answer to life the universe and everything, BUT it's not the secret number ...");
            continue;
        }

        let mut accuracy = guess.compare_to(&secret_number);
        if accuracy == Accuracy::Perfect {
            println!("You got it after {} tries! Well done!", number_of_guesses);
            break;
        }

        let response_option = random_number(0, 2);
        if response_option == 1 { accuracy = accuracy.inverse(); }

        let comparison = accuracy_responses[&accuracy];
        if response_option == 0 {
            println!("Your guess is {} than the secret number.", comparison);
        } else {
            println!("The secret number is {} than your guess.", comparison);
        }

        guess = prompt(random_prompt_string(), secret_range_from, secret_range_to_excl);
    }
}
