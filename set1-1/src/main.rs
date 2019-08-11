extern crate base64;
extern crate hex;

const INPUT : &str = "49276d206b696c6c696e6720796f757220627261696e206c696b65206120706f69736f6e6f7573206d757368726f6f6d";
const OUTPUT : &str = "SSdtIGtpbGxpbmcgeW91ciBicmFpbiBsaWtlIGEgcG9pc29ub3VzIG11c2hyb29t";

fn main() {
    let parsed = hex::decode(INPUT)?;
    assert_eq!(base64::encode(&parsed).as_str(), OUTPUT);
    println!("OK");
}