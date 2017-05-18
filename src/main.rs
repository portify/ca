use std::io::{self, BufRead, Write};

mod parser;
mod tokenizer;

use tokenizer::Tokenizer;

fn main() {
	let mut line = String::new();
	let stdin = io::stdin();

	loop {
		print!("% ");
		io::stdout().flush().ok().expect("Could not flush stdout");
		stdin.lock().read_line(&mut line).expect("Could not read line");
		if line.len() == 0 { break }
		let tokens = line.tokenize();
		let expression = parser::parse(tokens);
		println!("{:?}", expression);
		line.truncate(0)
	}
}
