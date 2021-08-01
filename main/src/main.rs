use base58::base58::{Encoder, Decoder};

fn main() {
    let bs58 = "E2DKDDG";
    println!("{:?}", bs58.encode());

    let strc = "3d6BsgcGmC";
    println!("{:?}", strc.decode().unwrap());
}
