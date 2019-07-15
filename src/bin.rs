use gfyhash::hash;

use std::env;

const ANIMALS: &'static str = include_str!("../lists/animals.txt");
const ADJECTIVES: &'static str = include_str!("../lists/adjectives.txt");

fn main() {
	let adjectives: Vec<&str> = ADJECTIVES.lines().collect();
	let animals: Vec<&str> = ANIMALS.lines().collect();

	let string = env::args().last().unwrap();

	println!("{}", hash::hash(string, 2, adjectives, animals));
}