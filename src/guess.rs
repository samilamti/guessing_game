use std::cmp::Ordering;

pub struct Guess {
    value: u16
}

#[derive(PartialEq, PartialOrd, Eq, Hash)]
#[derive(Debug)]
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

impl Guess { 
    pub fn new(value: u16) -> Guess {
        Guess { value }
    }

    pub fn value(&self) -> u16 {
        self.value
    }

    pub fn compare_to(&self, value: &u16) -> Accuracy {
        let diff = std::cmp::max(&self.value, &value) / std::cmp::min(&self.value, &value);
        
        match self.value.cmp(&value) {
            Ordering::Less => if diff > 3 {Accuracy::MuchLess} else if diff > 2 {Accuracy::LessThanHalf} else {Accuracy::Less},
            Ordering::Greater => if diff > 3 {Accuracy::MuchMore} else if diff > 2 {Accuracy::MoreThanDouble} else {Accuracy::More},
            Ordering::Equal => Accuracy::Perfect
        }
    }
}