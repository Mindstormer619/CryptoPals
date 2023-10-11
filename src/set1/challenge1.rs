pub fn hex_to_base64(hex_string: &str) -> String {
	bytes_to_base64(&hex_to_bytes(hex_string))
}

pub fn hex_to_bytes(hex_string: &str) -> Vec<u8> {
	hex_string
		.as_bytes()
		.chunks(2)
		.map(|chunk| {
			// convert each chunk to a string
			let chunk_string = std::str::from_utf8(chunk).unwrap();
			// convert each chunk to a u8
			u8::from_str_radix(chunk_string, 16).unwrap()
		})
		.collect::<Vec<u8>>()
}

pub fn bytes_to_base64(bytes: &[u8]) -> String {
	// chunk the bytes into sets of 3
	return bytes.chunks(3)
		.map(|chunk| {
			// convert 3 byte chunk into 4 chunks of 6 bits each
			let chunk = chunk.iter().fold(0, |acc, &byte| {
				acc << 8 | byte as u32
			});
			// convert each 6 bit chunk into a base64 character
			(0..4).map(|i| {
				let six_bit_chunk = (chunk >> (18 - 6 * i)) & 0b111111;
				match six_bit_chunk {
					0..=25 => (six_bit_chunk + 65) as u8 as char,
					26..=51 => (six_bit_chunk + 71) as u8 as char,
					52..=61 => (six_bit_chunk - 4) as u8 as char,
					62 => '+',
					63 => '/',
					_ => panic!("Invalid six bit chunk: {}", six_bit_chunk),
				}
			}).collect::<String>()
		}).collect::<Vec<String>>().join("")
}

#[cfg(test)]
mod tests {
	use crate::set1::challenge1::hex_to_bytes;

	#[test]
	fn verify_if_chunks_of_bytes_works() {
		let num_of_chunks =
			"49276d206b696c6c696e6720796f757220627261696e206c69\
			6b65206120706f69736f6e6f7573206d757368726f6f6d"
			.as_bytes()
			.chunks(3)
			.count();
		assert_eq!(96/3, num_of_chunks)
	}

	#[test]
	fn verify_if_hex_to_bytes_works() {
		let hex_string = "492af";
		// pairwise iterate over hex_string, taking 2 characters at a time
		let bytes = hex_to_bytes(hex_string);
		assert_eq!(vec![73, 42, 15], bytes);
	}

	#[test]
	fn verify_if_bytes_to_base64_works() {
		let bytes = vec![73, 42, 15];
		let base64_string = "SSoP";
		assert_eq!(base64_string, super::bytes_to_base64(&bytes));
	}

	#[test]
	fn verify_if_hex_to_base64_works() {
		let hex_string =
			"49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f\
			69736f6e6f7573206d757368726f6f6d";
		let base64_string =
			"SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29\
			ub3VzIG11c2hyb29t";
		assert_eq!(base64_string, super::hex_to_base64(hex_string));
	}
}