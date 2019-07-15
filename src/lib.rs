pub mod hash {
	use crc::crc32::checksum_ieee;
	// This only does strings, as that's the only data I need it to do.
	// The basic idea is to checksum the string, take bits of the
	// checksum then use those to grab lines from the text.
	pub fn hash(data: String, len: u32, adjs: Vec<&str>, animals: Vec<&str>) -> String {
		let sum = checksum_ieee(data.as_bytes());
		let mut hash = String::from("");
		for i in 1..(len + 1) {
			hash.push_str(adjs[((sum as u64 * i as u64) % adjs.len() as u64) as usize]);
		}
		hash.push_str(animals[(sum % animals.len() as u32) as usize]);
		hash
	}
}