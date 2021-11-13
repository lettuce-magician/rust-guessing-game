use rand::Rng;
use clearscreen::clear;
use std::{
    io,
    cmp::Ordering,
};

pub struct Game {
    pub secret_number: u32
}

impl Game {
    pub fn start(&mut self) {
        loop {
            println!("Guessing game!");
            println!("Insert a number between 1 and 100: ");
    
            let mut guess = String::new();
    
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to guess the line.");
    
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
    
            match guess.cmp(&self.secret_number) {
                Ordering::Less => println!("Your number is too small."),
                Ordering::Greater => println!("Your number is too big."),
                Ordering::Equal => {
                    println!("You won!");
                    println!("Restarting... (Use Ctrl+C or Command+C to quit anytime)");

                    match clear() {
                        Ok(_) => {
                            self.secret_number = rand::thread_rng().gen_range(1..=100);
                            continue;
                        },
                        Err(_) => println!("Couldn't clear the screen."),
                    }
                },
            }
        }
    }
}