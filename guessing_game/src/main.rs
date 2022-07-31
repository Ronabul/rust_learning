use std::io;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
	
    println!("Guess the number!");
	
	let secret_num = rand::thread_rng().gen_range(1..=100);
	
    loop {
		
		println!("Please input your guess.");
		let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
            
        let guess: u32 = guess.trim().parse().expect("Please Type num"); 

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("Nice.");
                break;
            }
        }

        println!("You guessed: {guess}");
    }
}