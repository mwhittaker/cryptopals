#![feature(macro_rules)]

pub mod set1;

fn main() {
    use set1::challenge1::{ToHex, FromHex};

    let hex  = String::from_str("0123456789abcdefABCDEF");
    let bits = hex.to_hex().expect("");
    let s: String = FromHex::from_hex(bits).expect("");
    println!("{}", s);
}
