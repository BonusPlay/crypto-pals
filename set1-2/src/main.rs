extern crate hex;

const INPUT1: &str = "1c0111001f010100061a024b53535009181c";
const INPUT2: &str = "686974207468652062756c6c277320657965";
const OUTPUT: &str = "746865206b696420646f6e277420706c6179";

fn main() {
    let parsed1 = hex::decode(INPUT1)?;
    let parsed2 = hex::decode(INPUT2)?;
    let mut out: Vec<u8> = [].to_vec();

    for (i, byte) in parsed1.iter().enumerate() {
        out.push(byte ^ parsed2.get(i)?)
    }

    assert_eq!(hex::encode(out), OUTPUT);
    print!("OK");
}