use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    let mut input = String::new();

    let (start, end) = (1, 100);
    let random_number = rand::rng().random_range(start..=end);

    println!("Guess the random number!");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    let input: u32 = input.trim().parse().expect("Please type a number!");

    match input.cmp(&random_number) {
        Ordering::Less => println!("To Big!"),
        Ordering::Greater => println!("To small!"),
        Ordering::Equal => println!("Match!"),
    }

    println!("You guessed: {input}, the number was {random_number}");
}
