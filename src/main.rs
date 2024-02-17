use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Get user input

    println!("Enter an input:");
    let mut user_input: String = String::new();
    io::stdin().read_line(&mut user_input).expect("Error!");
    let user_input: u8 = user_input.trim().parse().expect("Input must be a number!");

    // Generate CPU number

    let cpu_number: u8 = rand::thread_rng().gen_range(1..=100);

    // Compare user_input with cpu_number

    match user_input.cmp(&cpu_number) {
        Ordering::Less => println!("{} is too small!", user_input),
        Ordering::Equal => println!("{} is correct!", user_input),
        Ordering::Greater => println!("{} is too big!", user_input),
    }
    println!("The number was {}! Sorry, buddy...", cpu_number);
}
