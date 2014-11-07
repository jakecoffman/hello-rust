#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

// use regex::Regex;
use std::rand;
use std::rand::Rng;

fn main() {
	// old and busted
	// let re = match Regex::new(r"^\d+$") {
	// 	Ok(re) => re,
	// 	Err(err) => panic!("{}", err),
	// };

	// new hotness
	let re = regex!(r"^\d+$");

	match re.is_match("123") {
		true => println!("That is correct"),
		false => panic!("INSANE IN THE MEMBRANE")
	}

	match may_return_none() {
		Some(val) => println!("{}", val),
		None => println!("returned None"),
	}

	let thingy = (1i, true);
	println!("{}", thingy)

	let mut v = 0i;
	println!("{}", v)
	v = v - 1;
	println!("{}", v)
}

fn may_return_none() -> Option<int> {
	let mut rng = rand::task_rng();
	let val = rng.gen::<int>(); 
	if val % 2 == 0 {
		return Some(val);
	} else {
		return None
	}
}

