use std::iter::Peekable;
use std::str::Chars;

use num::{pow, BigInt, BigRational};
use num::bigint::ToBigInt;

#[derive(Debug, PartialEq)]
pub enum Symbol {
	Add,
	Subtract,
	Multiply,
	Divide,
	Modulus,
	Exponent,
	Equals
}

#[derive(Debug, PartialEq)]
pub enum Token {
	Integer(BigRational),
	Name(String),
	Operator(Symbol)
}

pub trait Tokenizer {
	fn tokenize(&self) -> Vec<Token>;
}

impl Tokenizer for String {
	fn tokenize(&self) -> Vec<Token> {
		let mut it = self.chars().peekable();
		let mut tokens: Vec<Token> = vec![];

		loop {
			match it.peek() {
				Some(&ch) => match ch {
					'0' ... '9' => {
						let num: Vec<char> = consume_while(&mut it, |a| a.is_numeric() || a == '_' || a == '.')
							.into_iter()
							.collect();
						tokens.push(Token::Integer(parse_number(num)));
					},
					'+' => {
						it.next().unwrap();
						tokens.push(Token::Operator(Symbol::Add));
					},
					'-' => {
						it.next().unwrap();
						tokens.push(Token::Operator(Symbol::Subtract));
					},
					'*' => {
						it.next().unwrap();
						tokens.push(Token::Operator(Symbol::Multiply));
					},
					'/' => {
						it.next().unwrap();
						tokens.push(Token::Operator(Symbol::Divide));
					},
					'%' => {
						it.next().unwrap();
						tokens.push(Token::Operator(Symbol::Modulus));
					},
					'^' => {
						it.next().unwrap();
						tokens.push(Token::Operator(Symbol::Exponent));
					},
					'=' => {
						it.next().unwrap();
						tokens.push(Token::Operator(Symbol::Equals));
					},
					'\n' | '\t' | ' ' => {
						it.next().unwrap();
					},
					a if a.is_alphabetic() => {
						let name: String = consume_while(&mut it, |a| a.is_alphabetic())
							.into_iter()
							.collect();
						tokens.push(Token::Name(name));
					}
					_ => panic!("invalid char {}", ch)
				},
				None => break
			}
		}

		tokens
	}
}

fn parse_number(chars: Vec<char>) -> BigRational {
	// BigRational::new(chars.parse::<BigInt>().unwrap(). 1.to_bigint().unwrap())

	let mut separator: Option<usize> = None;
	let mut digits: Vec<char> = vec![];

	for a in chars.into_iter() {
		if a == '.' {
			separator = Some(digits.len());
		} else if a.is_numeric() {
			digits.push(a);
		}
	}

	let scale = match separator {
		Some(i) => digits.len() - i,
		None => 0
	};

	let denom = pow(10.to_bigint().unwrap(), scale);

	let digits: String = digits.into_iter().collect();

	BigRational::new(digits.parse::<BigInt>().unwrap(), denom)
}

fn consume_while<F>(it: &mut Peekable<Chars>, x: F) -> Vec<char>
	where F : Fn(char) -> bool {

	let mut v: Vec<char> = vec![];

	while let Some(&ch) = it.peek() {
		if x(ch) {
			it.next().unwrap();
			v.push(ch);
		} else {
			break;
		}
	}

	v
}
