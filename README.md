[中文](README_zh.md)

## Features

 * Simplea Trait
 * Fast and lightweight
 * Suitable for password-making

## base58 algorithm

**Wikipedia:**

the free encyclopedia Base58 is a group of binary-to-text encoding schemes used to represent large integers as alphanumeric text. It is similar to Base64 but has been modified to avoid both non-alphanumeric characters and letters which might look ambiguous when printed. It is therefore designed for human users who manually enter the data, copying from some visual source, but also allows easy copy and paste because a double-click will usually select the whole string.

Compared to Base64, the following similar-looking letters are omitted: 0 (zero), O (capital o), I (capital i) and l (lower case L) as well as the non-alphanumeric characters + (plus) and / (slash). In contrast to Base64, the digits of the encoding do not line up well with byte boundaries of the original data. For this reason, the method is well-suited to encode large integers, but not designed to encode longer portions of binary data. The actual order of letters in the alphabet depends on the implementation, which is the reason why the term “Base58” alone is not enough to fully describe the format. A variant, Base56, excludes 1 (one) and o (lowercase o) compared to Base 58.

## Traits
2 traits:
```
// encode to base58 string
pub trait Encoder {
	fn encode_to_base58(&self) -> String;
}

// Decode from base58 string
pub trait Decoder {
	fn decode_from_base58(&self) -> Result<String , DecodeError>;
}
```

## Usage

```toml
[dependencies]
base58  = {git = "https://github.com/QMHTMY/base58/base58" }
```

```rust
use base58::base58::{Encoder, Decoder};

fn main() {
    let bs58 = "E2DKDDG";
    println!("{:?}", bs58.encode_to_base58());

    let strc = "3d6BsgcGmC";
    println!("{:?}", strc.decode_from_base58().unwrap());
}
```

## LICENSE

Under the [Apache](LICENSE).
