use gfyhash::gfyhash;

use std::env;

const ANIMALS: &'static str = include_str!("../lists/animals.txt");
const ADJECTIVES: &'static str = include_str!("../lists/adjectives.txt");

fn main() {
	let adjectives: Vec<String> = ADJECTIVES.lines().map(|s| s.to_string()).collect();
	let animals: Vec<String> = ANIMALS.lines().map(|s| s.to_string()).collect();

	let string = env::args().last().unwrap();

	println!("{}", gfyhash(&string, None, None, None));
	println!("{}", gfyhash(&string, 2, adjectives, animals))
}