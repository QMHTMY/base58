[English](README.md)

## 特点

 * 语法简单
 * 速度快、安全
 * 可用于生成密码

## base58 算法

类似 base64 编码算法， 但去掉了几个看起来相同的字符（数字0、大写字母O, 大写字母I, 字母 l），以及特殊字符（+，/）。只含有字母、数字。优点是不易看错字符，且在大部分字符显示场景中，不会断行，可以双击复制。

## Traits
分别提供了两个 Trait
```
// 编码
pub trait Encoder {
	fn encode_to_base58(&self) -> String;
}

// 解码
pub trait Decoder {
	fn decode_from_base58(&self) -> Result<String, DecodeError>;
}
```

## 使用
```toml
[dependencies]
base58  = {git = "https://github.com/QMHTMY/base58/base58" }
```

```rust
use base58::base58::{Encoder, Decoder};

fn main() {
    let bs58 = "E2DKDDG";
    println!("{:?}", bs58.().encode_to_base58());

    let strc = "3d6BsgcGmC";
    println!("{:?}", strc.decode_from_base58().unwrap());
}
```

## 协议
基于 [Apache License](LICENSE) 协议。
