use std::io::{self, Write};
use rand::Rng;
use std::collections::HashMap;
use crate::secret_number::SecretNumber;
use crate::secret_number::Accuracy;

mod secret_number;

fn prompt(question: String, secret_range_from: u16, secret_range_to_excl: u16) -> u16 {
    loop {
        print!("{}", question);
        io::stdout().flush().unwrap();

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        
        match guess.trim().parse::<u16>() {
            Ok(num) if secret_range_from <= num && num < secret_range_to_excl => return num,
            Ok(_) => {
                println!("Nice guess, but the secret number is between {} and {} ... ;)", secret_range_from, secret_range_to_excl);
                continue;
            },
            Err(_) => {
                println!("Please guess a number between {} and {} ... ;)", secret_range_from, secret_range_to_excl);
                continue;
            },
        };
    }
}

fn random_prompt_string() -> String {
    let random_number = rand::thread_rng().gen_range(0, 2);
    let variants = vec!["Guess again: ", "You next guess? "];
    let random_string = variants[random_number as usize].to_string();
    return random_string;
}

fn main() {
    println!("Guess the number! ");

    let mut secret_number = SecretNumber::new();
    let secret_range_from = secret_number.range_from();
    let secret_range_to_excl = secret_number.range_to_excl();

    let mut accuracy_responses = HashMap::new();
    accuracy_responses.insert(Accuracy::More, "larger");
    accuracy_responses.insert(Accuracy::MoreThanDouble, "more than double");
    accuracy_responses.insert(Accuracy::MuchMore, "a lot larger");
    accuracy_responses.insert(Accuracy::Less, "smaller");
    accuracy_responses.insert(Accuracy::LessThanHalf, "less than half");
    accuracy_responses.insert(Accuracy::MuchLess, "a lot smaller");

    let mut guess = prompt(format!("Guess a number between {} and {}: ", secret_range_from, secret_range_to_excl), secret_range_from, secret_range_to_excl); 
    loop {        
        let accuracy = &secret_number.guess(&guess);
        if accuracy == &Accuracy::Perfect {
            println!("You got it after {} tries! Well done!", secret_number.number_of_guesses());
            break;
        }

        if guess == 42 {
            println!("42 is the answer to life the universe and everything, BUT it's not the secret number ...");
            continue;
        }

        match rand::thread_rng().gen_range(0, 2) {
            0 => println!("Your guess is {} than the secret number.", accuracy_responses[&accuracy.inverse()]),
            _ => println!("The secret number is {} than your guess.", accuracy_responses[&accuracy])
        };

        guess = prompt(random_prompt_string(), secret_range_from, secret_range_to_excl);
    }
}
