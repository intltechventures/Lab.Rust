/**
Reference
http://doc.rust-lang.org/nightly/book/guessing-game.html
*/

use std::io;

fn main() {
	println!("Guess the number:");
	
	println!("Please input your guess.");
	
	//  Rust’s variable bindings are immutable by default.
	// mut makes a binding mutable, rather than immutable
	let mut guess = String::new();
	
	io::stdin().read_line(&mut guess)
		.expect("failed to read line");
	
	println!("You guess: {}", guess);
	
}
