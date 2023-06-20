use std::io;
use std::cmp::Ordering;
use std::num::ParseIntError;
use rand::Rng;

fn main() {
    let random_number = rand::thread_rng().gen_range(0..=100);

    println!("Guess a number:");

    loop {
        let guessed_number = match user_guessed_number() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match guessed_number.cmp(&random_number) {
            Ordering::Less => println!("The secret number is greater than {guessed_number}"),
            Ordering::Greater => println!("The secret number is less than {guessed_number}"),
            Ordering::Equal => {
                println!("You've guessed right");
                break;
            },
        }
    }
}

fn user_guessed_number() -> Result<u32, ParseIntError> {
    let mut guessed_number = String::new();
    io::stdin().read_line(&mut guessed_number).expect("Error while waiting for user input");
    
    guessed_number.trim().parse()
}

