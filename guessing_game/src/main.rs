use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("{secret_number}");

    println!("Hello, and welcome to the guessing game!");

    let mut guess;

    loop {
        println!("Please enter your input:");

        guess = String::new();

        io::stdin().read_line(&mut guess).expect("Failed to read line!");

        println!("{guess}");

        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => continue
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("Good job, the number are equal.");
                break;
            }
        }
    }

    println!("You guessed: {guess}");
}
