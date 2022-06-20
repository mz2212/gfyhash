//! This crate is a way to represent anything that implements [`Hashable`] as a (variable length) AdjectiveAdjectiveAnimal sequence.
//! 
//! My inspiration comes from the website [gfycat](https://gfycat.com/) as you probably gathered from the name of the crate.
//! It's probably worth noting that this "hash" is not cryptographically secure, and should not be used as such.
//! 
//! Thanks to [a-type](https://github.com/a-type/adjective-adjective-animal)'s repo for the builtin wordlists.
//! 
//! # Examples
//! ## Print a "gfyhashed" String
//! ```rust
//! use gfyhash::gfyhash;
//! let foobar = String::from("foobar");
//! println!("{}", gfyhash(&foobar, None, None, None));
//! ```
//! 
//! # Feature Flags
//! * `list_builtin` (default) - Enables the use of the builtin wordlists. These can be found in the lists subdirectory at the root of the repo.

use wyhash::wyhash;
#[cfg(feature = "list_builtin")]
const ADJECTIVES: &'static str = include_str!("../lists/adjectives.txt");
#[cfg(feature = "list_builtin")]
const ANIMALS: &'static str = include_str!("../lists/animals.txt");


/// WyHash the string, then use that to create the "gfyhashed" representation
/// 
///  # Arguments
/// 
/// * `data` - Data to be "gfyhashed", implements [`Hashable`]
/// * `len` - Count of adjectives. I.E. 1 == AdjectiveAnimal. Pass [`None`] for the default of 2
/// * `adjectives` - [`Vec`] of [`String`]s to be used as the "adjective" list. Pass [`None`] with the `list_builtin` feature enabled for the default list
/// * `animals` - Same story as adjectives
pub fn gfyhash(data: &impl Hashable, len: impl Into<Option<usize>>, adjectives: impl Into<Option<Vec<String>>>, animals: impl Into<Option<Vec<String>>>) -> String {
	let len = len.into().unwrap_or(2);
	
	#[cfg(feature = "list_builtin")]
	let adjectives = adjectives.into().unwrap_or(ADJECTIVES.lines().map(|s| s.to_string()).collect());
	#[cfg(feature = "list_builtin")]
	let animals = animals.into().unwrap_or(ANIMALS.lines().map(|s| s.to_string()).collect());
	
	#[cfg(not(feature = "list_builtin"))]
	let adjectives = adjectives.into().unwrap();
	#[cfg(not(feature = "list_builtin"))]
	let animals = animals.into().unwrap();
	
	
	let seed = 42; // I'm not entirely sure what seed does, but I'm sure you get why I chose it
	let sum = wyhash(data.as_bytes(), seed) as usize;
	let mut hash = String::from("");
	for i in 1..=len {
		hash += &(adjectives[(sum.wrapping_mul(i)) % adjectives.len()]);
	}
	hash += &(animals[sum % animals.len()]);
	hash
}

/// Anything that can be represented as an array of bytes that someone would like to be "gfyhashed"
pub trait Hashable {
	fn as_bytes(&self) -> &[u8];
}

impl Hashable for std::string::String {
	fn as_bytes(&self) -> &[u8] {
		self.as_bytes()
	}
}

impl Hashable for &[u8] {
	fn as_bytes(&self) -> &[u8] {
		self
	}
}