use std::{cmp::Ordering, io};

use rand::Rng;

fn main(){
	guess_game();
}

fn guess_game(){
	println!("Guess the number");
	println!("Please input your guess.");
	// generate random number between 1 and 100 
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");

	//declare a mutable string to read from the stdin
	let mut guess = String::new();
	io::stdin()
		.read_line(&mut guess)
		.expect("Failed to read line");
	//convert the string just read as an integer
	let guess: u32 = guess.trim().parse().expect("Please type a number");
	println!("You guessed: {guess}");

	//compare the typep and the random number
	match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!"),
    }
}