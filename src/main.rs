extern crate rand;

use std::io; //use a thing called io(input/output) from the std(standard) library
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");
	
	let secret_number = rand::thread_rng().gen_range(1, 101);
	
	//println!("The secret number is: {}", secret_number);
	
	loop {
	
		println!("Please input your guess.");
	
		let mut guess = String::new();
	
		io::stdin().read_line(&mut guess)
			.expect("Failed to read line");
		
		let guess: u32 = match guess.trim().parse() {//this changes guess from a string to an int(unsigned32bit)
			Ok(num) => num,
			Err(_) => continue,
		};	
	
		println!("You guessed: {}", guess);
	
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

