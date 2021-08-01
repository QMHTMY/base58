pub mod base58;

#[cfg(test)]
mod tests {
	use crate::base58::{Encoder, Decoder};

	#[test]
	fn test_decode_basic() {
		assert_eq!("x".decode().unwrap(), "7");
		assert_eq!("j".decode().unwrap(), "*");
		assert_eq!("p".decode().unwrap(), "/");
	}

	#[test]
	fn test_encode_basic() {
		assert_eq!("".encode(), "");
		assert_eq!("x".encode(), "35");
		assert_eq!("j".encode(), "2q");
		assert_eq!("p".encode(), "2w");
		assert_eq!("pjx".encode(), "em3D");
	}
}
