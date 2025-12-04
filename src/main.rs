use std::io;

fn main() {
    let mut input = String::new();

    println!("Input: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line.");

    println!("{input}");
}
