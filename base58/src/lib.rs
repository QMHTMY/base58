pub mod base58;

#[cfg(test)]
mod tests {
	use crate::base58::{Encoder, Decoder};

	#[test]
	fn test_decode_basic() {
		assert_eq!("x".decode_from_base58().unwrap(), "7");
		assert_eq!("j".decode_from_base58().unwrap(), "*");
		assert_eq!("p".decode_from_base58().unwrap(), "/");
	}

	#[test]
	fn test_encode_basic() {
		assert_eq!("".encode_to_base58(), "");
		assert_eq!("x".encode_to_base58(), "35");
		assert_eq!("j".encode_to_base58(), "2q");
		assert_eq!("p".encode_to_base58(), "2w");
		assert_eq!("pjx".encode_to_base58(), "em3D");
	}
}
