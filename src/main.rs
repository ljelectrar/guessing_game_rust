use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

    //processing a guess
    println!("Guess the number, baby!");
    //println!("Please, input using the keyboard your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // 'shadow' // casting \)(O)
    let guess: u32 = guess.trim().parse().expect("Please, type a number!");

    println!("You guess: {guess}");

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Equal => println!("You win!"),
        Ordering::Greater => println!("Too big"),
    }
}
