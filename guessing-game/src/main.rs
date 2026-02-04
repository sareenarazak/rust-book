use ::rand;
use std::cmp::Ordering;
use std::io;

fn main() {
    let secret_number = rand::random_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("guess a number!");

    loop {
        println!("input your guess");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("invalid input, enter a number");
                continue;
            }
        };

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("too smol"),
            Ordering::Equal => {
                println!("yay you won!");
                break;
            }
            Ordering::Greater => println!("too big"),
        }
    }
}
