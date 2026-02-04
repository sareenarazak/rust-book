// `use` to import
use std::cmp::Ordering;
use std::io;

fn main() {
    // declare a new mutable variable
    // use mut for mutable variables -- rust variables are immutable
    let mut something = String::new();

    // printing a variable {} placeholder
    println!("something printed {something}");

    // match -- matches the pattern and returns enum Result
    // parse returns Ok or Err

    let first = 1;
    let second = 2;
    match first.cmp(&second) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    };

    // loops -- infinite
    let mut count = 1;

    loop {
        if count == 2 {
            break;
        }
        
        count += 1;
        println!("keep looping");
    
    }
}
