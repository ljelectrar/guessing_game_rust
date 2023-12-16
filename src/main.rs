use std::io;

fn main() {

    //processing a guess
    println!("Guess the number, baby!");
    println!("Please, input using the keyboard your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guess: {guess}");

}
