pub mod base58;

#[cfg(test)]
mod tests {
	use crate::base58::{ToBase58, FromBase58};

	#[test]
	fn test_from_base58_basic() {
		assert_eq!("".from_base58().unwrap(), b"");
		assert_eq!("x".from_base58().unwrap(), &[55]);
		assert_eq!("j".from_base58().unwrap(), &[42]);
		assert_eq!("p".from_base58().unwrap(), &[47]);
		assert_eq!("xjp".from_base58().unwrap(), &[2, 220, 111]);
		assert_eq!("xwn".from_base58().unwrap(), &[2, 223, 37]);
		assert_eq!("3YAUaAyfR3ps6".from_base58().unwrap(), b"wechatxjp");
	}

	#[test]
	fn test_from_base58_invalid_char() {
		assert!("1".from_base58().is_err());
		assert!("0".from_base58().is_err());
		assert!("o".from_base58().is_err());
		assert!("O".from_base58().is_err());
		assert!("I".from_base58().is_err());
		assert!("l".from_base58().is_err());
		assert!("3mJr0".from_base58().is_err());
		assert!("s!5<".from_base58().is_err());
		assert!("t$@mX<*".from_base58().is_err());
	}

	#[test]
	fn test_to_base58_basic() {
		assert_eq!(b"".to_base58(), "");
		assert_eq!(&[55].to_base58(), "x");
		assert_eq!(&[42].to_base58(), "j");
		assert_eq!(&[47].to_base58(), "p");
		assert_eq!(b"xjp".to_base58(), "iTvY");
	}
}
