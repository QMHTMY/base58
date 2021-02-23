[English](README.md)

## 特点

 * 语法简单
 * 速度快、安全
 * 主要用于生成密码

## base58 算法

类似 base64 编码算法， 但去掉了几个看起来相同的字符（数字0、小写字母o、大写字母O, 大写字母I, 字母l和数字1），以及非字母数字字符（+，/）。只含有字母、数字。优点是不易看错字符，且在大部分字符显示场景中，可以双击复制。为生成密码，特意添加了 * 和 _ 两个符号。

## Traits
分别提供了两个 Trait
```
// 编码
pub trait ToBase58 {
	fn to_base58(&self) -> String;
}

// 解码
pub trait FromBase58 {
	fn from_base58(&self) -> Result<Vec<u8>, FromBase58Error>;
}
```

## 使用
```toml
[dependencies]
base58  = {git = "https://github.com/QMHTMY/base58/base58" }
```

```rust
use base58::base58::{ToBase58, FromBase58};

fn main() {
    let bs58 = "E2DKDDG";
    println!("{:?}", bs58.from_base58().unwrap());

    let strc = b"wechatxjp";
    println!("{:?}", strc.to_base58());
}
```

## 协议
基于 [Apache License](LICENSE) 协议。
