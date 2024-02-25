use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");

    // Get Secret Number 
    let secret_number = rand::thread_rng().gen_range(1..=100);

    // Loop Initialization
    loop {
        println!("Please input your guess.");

        // Mutable Variable Declaration
        let mut guess = String::new();

        // Read Line and Handling Potential Failure with Result
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // Handling Invalid Input
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {guess}");

        // Find Matching 
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}