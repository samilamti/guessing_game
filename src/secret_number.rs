use std::cmp::{Ordering, min, max};
use rand::Rng;

pub struct SecretNumber {
    value: u16,
    secret_range_from: u16,
    secret_range_to_excl: u16,
    guess_counter: u16,

}

#[derive(PartialEq, PartialOrd, Eq, Hash, Debug)]
pub enum Accuracy {
    MuchLess,
    LessThanHalf,
    Less,
    Perfect,
    More,
    MoreThanDouble,
    MuchMore
}

impl Accuracy {
    pub fn inverse(&self) -> Accuracy {
        return match &self {
            Accuracy::MuchLess => Accuracy::MuchMore,
            Accuracy::LessThanHalf => Accuracy::MoreThanDouble,
            Accuracy::Less => Accuracy::More,
            Accuracy::Perfect => Accuracy::Perfect,
            Accuracy::More => Accuracy::Less,
            Accuracy::MoreThanDouble => Accuracy::LessThanHalf,
            Accuracy::MuchMore => Accuracy::MuchLess
        }
    }
}

impl SecretNumber { 
    pub fn new() -> SecretNumber { 
        let mut rng = rand::thread_rng();
        let secret_range_from = rng.gen_range(1, 101);
        let secret_range_multiplier = rng.gen_range(3, 6);
        let secret_range_to_excl = secret_range_from * secret_range_multiplier;
        let secret_number = rng.gen_range(secret_range_from, secret_range_to_excl);
    
        SecretNumber { value: secret_number, secret_range_from, secret_range_to_excl, guess_counter: 0 }
    }

    pub fn number_of_guesses(&self) -> u16 {
        self.guess_counter
    }

    pub fn range_from(&self) -> u16 {
        self.secret_range_from
    }

    pub fn range_to_excl(&self) -> u16 {
        self.secret_range_to_excl
    }

    pub fn guess(&mut self, value: &u16) -> Accuracy {
        self.guess_counter += 1;
        let diff = max(&self.value, &value) / min(&self.value, &value);
        
        match self.value.cmp(&value) {
            Ordering::Less => if diff > 3 {Accuracy::MuchLess} else if diff > 2 {Accuracy::LessThanHalf} else {Accuracy::Less},
            Ordering::Greater => if diff > 3 {Accuracy::MuchMore} else if diff > 2 {Accuracy::MoreThanDouble} else {Accuracy::More},
            Ordering::Equal => Accuracy::Perfect
        }
    }
}