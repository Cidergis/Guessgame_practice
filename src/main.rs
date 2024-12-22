use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Welcome to guessing game practice project!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    //println!("The secret number is: {secret_number}");

    loop {
        println!("Please guess a number! OR ELSE!");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too smol!"),
            Ordering::Greater => println!("Too beeg!"),
            Ordering::Equal => {
                println!("GG");
                break;
            }
        }
    }
}
