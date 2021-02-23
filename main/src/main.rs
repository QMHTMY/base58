use base58::base58::{ToBase58, FromBase58};

fn main() {
    let bs58 = "E2DKDDG";
    println!("{:?}", bs58.from_base58().unwrap());

    let strc = b"wechatxjp";
    println!("{:?}", strc.to_base58());
}
